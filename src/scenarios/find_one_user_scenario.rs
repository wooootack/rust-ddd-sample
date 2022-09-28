use crate::{
  modules::{
    common::{api_error::ApiError, date::date_format},
    user::presentation::find_one_user::{self, FindOneUserRequest, UserResponse},
  },
  scenarios::request_context::RequestContext,
};
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserScenarioResponse {
  pub id: String,
  pub first_name: String,
  pub last_name: String,
  pub mail_address: String,
  pub age: i16,
  #[serde(with = "date_format")]
  pub created_at: DateTime<Utc>,
  #[serde(with = "date_format")]
  pub updated_at: DateTime<Utc>,
}

impl From<UserResponse> for UserScenarioResponse {
  fn from(response: UserResponse) -> Self {
    UserScenarioResponse {
      id: response.id,
      first_name: response.first_name,
      last_name: response.last_name,
      mail_address: response.mail_address,
      age: response.age,
      created_at: response.created_at,
      updated_at: response.updated_at,
    }
  }
}

pub async fn execute(
  context: web::Data<RequestContext>,
  user_id: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
  let request = FindOneUserRequest {
    id: user_id.to_string(),
  };

  let response = web::block(move || {
    let conn = context.get_connection();

    find_one_user::execute(conn, request)
  })
  .await??;

  match response {
    Some(response) => Ok(HttpResponse::Ok().json(UserScenarioResponse::from(response))),
    None => Ok(HttpResponse::Ok().body("User not found")),
  }
}
