use crate::{
  modules::{
    common::{api_error::ApiError, date::date_format},
    task::presentation::create_task::{self, CreateTaskRequest, TaskResponse},
  },
  scenarios::request_context::RequestContext,
};
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateTaskScenarioRequest {
  pub title: String,
  pub body: String,
}

#[derive(Serialize)]
pub struct TaskScenarioResponse {
  pub id: String,
  pub title: String,
  pub body: String,
  pub user_id: String,
  #[serde(with = "date_format")]
  pub created_at: DateTime<Utc>,
  #[serde(with = "date_format")]
  pub updated_at: DateTime<Utc>,
}

impl From<TaskResponse> for TaskScenarioResponse {
  fn from(response: TaskResponse) -> Self {
    TaskScenarioResponse {
      id: response.id,
      title: response.title,
      body: response.body,
      user_id: response.user_id,
      created_at: response.created_at,
      updated_at: response.updated_at,
    }
  }
}

pub async fn execute(
  context: web::Data<RequestContext>,
  user_id: web::Path<String>,
  request: web::Json<CreateTaskScenarioRequest>,
) -> Result<HttpResponse, ApiError> {
  let request = CreateTaskRequest {
    title: request.title.to_string(),
    body: request.body.to_string(),
    user_id: user_id.to_string(),
  };

  let response = web::block(move || {
    let conn = context.get_connection();

    create_task::execute(conn, request)
  })
  .await??;

  Ok(HttpResponse::Ok().json(TaskScenarioResponse::from(response)))
}
