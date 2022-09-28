use crate::{
  modules::{
    common::api_error::ApiError,
    user::presentation::update_user::{self, UpdateUserRequest},
  },
  scenarios::{find_one_user_scenario::UserScenarioResponse, request_context::RequestContext},
};
use actix_web::{web, HttpResponse};
use serde::Deserialize;

use super::request_context::AuthUser;

#[derive(Deserialize)]
pub struct UpdateUserScenarioRequest {
  pub first_name: String,
  pub last_name: String,
  pub age: i16,
}

pub async fn execute(
  _req_user: AuthUser,
  context: web::Data<RequestContext>,
  user_id: web::Path<String>,
  request: web::Json<UpdateUserScenarioRequest>,
) -> Result<HttpResponse, ApiError> {
  let request = UpdateUserRequest {
    id: user_id.to_string(),
    first_name: request.first_name.to_string(),
    last_name: request.last_name.to_string(),
    age: request.age,
  };

  let res = web::block(move || {
    let conn = context.get_connection();
    update_user::execute(conn, request)
  })
  .await??;

  Ok(HttpResponse::Ok().json(UserScenarioResponse::from(res)))
}
