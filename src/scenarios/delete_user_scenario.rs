use crate::{
  modules::{
    common::api_error::ApiError,
    user::presentation::delete_user::{self, DeleteUserRequest},
  },
  scenarios::request_context::RequestContext,
};
use actix_web::{http::StatusCode, web, HttpResponse};

use super::request_context::AuthUser;

pub async fn execute(
  _req_user: AuthUser,
  context: web::Data<RequestContext>,
  user_id: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
  let request = DeleteUserRequest {
    id: user_id.to_string(),
  };

  web::block(move || {
    let conn = context.get_connection();

    delete_user::execute(conn, request)
  })
  .await??;

  Ok(HttpResponse::Ok().status(StatusCode::OK).finish())
}
