use crate::{
  modules::{
    common::api_error::ApiError,
    user::presentation::create_user::{self, CreateUserRequest},
  },
  scenarios::{find_one_user_scenario::UserScenarioResponse, request_context::RequestContext},
};
use actix_web::{web, HttpResponse};
use serde::Deserialize;

use super::request_context::AuthUser;

#[derive(Deserialize)]
pub struct CreateUserScenarioRequest {
  pub first_name: String,
  pub last_name: String,
  pub mail_address: String,
  pub age: i16,
}

pub async fn execute(
  _req_user: AuthUser,
  context: web::Data<RequestContext>,
  request: web::Json<CreateUserScenarioRequest>,
) -> Result<HttpResponse, ApiError> {
  let request = CreateUserRequest {
    first_name: request.first_name.to_string(),
    last_name: request.last_name.to_string(),
    mail_address: request.mail_address.to_string(),
    age: request.age,
  };

  let response = web::block(move || {
    let conn = context.get_connection();

    create_user::execute(conn, request)
  })
  .await??;

  Ok(HttpResponse::Ok().json(UserScenarioResponse::from(response)))
}
