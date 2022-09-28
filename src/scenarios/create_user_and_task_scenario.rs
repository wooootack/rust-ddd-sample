use super::request_context::AuthUser;
use super::{
  create_task_scenario::TaskScenarioResponse, find_one_user_scenario::UserScenarioResponse,
  request_context::RequestContext,
};
use crate::modules::{
  common::api_error::ApiError,
  task::presentation::create_task::{self, CreateTaskRequest, TaskResponse},
  user::presentation::{
    create_user::{self, CreateUserRequest},
    find_one_user::UserResponse,
  },
};
use actix_web::{error::BlockingError, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserAndTaskScenarioRequest {
  pub first_name: String,
  pub last_name: String,
  pub mail_address: String,
  pub age: i16,
  pub title: String,
  pub body: String,
}

#[derive(Serialize)]
pub struct UserAndTaskScenarioResponse {
  pub user: UserScenarioResponse,
  pub task: TaskScenarioResponse,
}

impl From<(UserResponse, TaskResponse)> for UserAndTaskScenarioResponse {
  fn from(response: (UserResponse, TaskResponse)) -> Self {
    UserAndTaskScenarioResponse {
      user: response.0.into(),
      task: response.1.into(),
    }
  }
}

trait Outputting: Sized {
  fn outputting<O>(self) -> Self
  where
    Self: std::future::Future<Output = O>,
  {
    self
  }
}
impl<T: std::future::Future> Outputting for T {}

pub async fn execute(
  _req_user: AuthUser,
  context: web::Data<RequestContext>,
  request: web::Json<CreateUserAndTaskScenarioRequest>,
) -> Result<HttpResponse, ApiError> {
  let response = web::block(move || {
    let conn = context.get_connection();

    let user_request = CreateUserRequest {
      first_name: request.first_name.to_string(),
      last_name: request.last_name.to_string(),
      mail_address: request.mail_address.to_string(),
      age: request.age,
    };
    let user_response = create_user::execute(conn, user_request)?;

    let conn = context.get_connection();
    let task_request = CreateTaskRequest {
      title: request.title.to_string(),
      body: request.body.to_string(),
      user_id: user_response.id.clone(),
    };
    let task_response = create_task::execute(conn, task_request)?;

    Ok((user_response, task_response))
  })
  .outputting::<Result<Result<(UserResponse, TaskResponse), ApiError>, BlockingError>>()
  .await??;

  Ok(HttpResponse::Ok().json(UserAndTaskScenarioResponse::from(response)))
}

// pub async fn execute_transactional(
//   _req_user: AuthUser,
//   context: web::Data<RequestContext>,
//   request: web::Json<CreateUserAndTaskScenarioRequest>,
// ) -> Result<HttpResponse, ApiError> {
//   let response = web::block(move || {
//     let conn = context.get_connection();

//     conn.transaction(|tx| {
//       let user_request = CreateUserRequest {
//         first_name: request.first_name.to_string(),
//         last_name: request.last_name.to_string(),
//         mail_address: request.mail_address.to_string(),
//         age: request.age,
//       };
//       let user_response = create_user::execute(tx, user_request)?;

//       let task_request = CreateTaskRequest {
//         title: request.title.to_string(),
//         body: request.body.to_string(),
//         user_id: user_response.id.clone(),
//       };
//       let task_response = create_task::execute(tx, task_request)?;

//       Ok((user_response, task_response))
//     })
//   })
//   .outputting::<Result<Result<(UserResponse, TaskResponse), ApiError>, BlockingError>>()
//   .await??;

//   Ok(HttpResponse::Ok().json(UserAndTaskScenarioResponse::from(response)))
// }
