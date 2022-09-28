use crate::{
  modules::{common::api_error::ApiError, diesel::util},
  scenarios::request_context::RequestContext,
};
use actix_web::{http::StatusCode, web, HttpResponse};
use std::env;

pub async fn execute(context: web::Data<RequestContext>) -> Result<HttpResponse, ApiError> {
  let has_pending_migration = web::block(move || {
    let conn = &mut context.get_connection();
    let migrations_dir = env::var("MIGRATIONS_DIR").expect("MIGRATIONS_DIR must be set");
    util::has_pending_migration(conn, &migrations_dir)
  })
  .await??;

  if has_pending_migration {
    Err(ApiError::new(
      StatusCode::INTERNAL_SERVER_ERROR,
      "適用されていないマイグレーションがあります".to_owned(),
    ))
  } else {
    Ok(HttpResponse::Ok().body("ok"))
  }
}
