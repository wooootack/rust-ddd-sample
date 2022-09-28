use crate::modules::common::api_error::ApiError;
use actix_web::{dev::Payload, http::StatusCode, FromRequest, HttpRequest};
use diesel::{
  r2d2::{self, ConnectionManager, Pool, PooledConnection},
  Connection, PgConnection,
};
use dotenvy::from_filename;
use serde::{Deserialize, Serialize};
use std::{env, future::Future, pin::Pin};

pub type DBConn = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Serialize, Deserialize)]
pub struct AuthUser {
  pub id: String,
  pub mail_address: String,
  pub role: String,
}

impl AuthUser {
  pub fn is_admin(&self) -> bool {
    self.role == "admin"
  }
  pub fn is_user(&self) -> bool {
    self.role == "user"
  }
}

impl FromRequest for AuthUser {
  type Error = ApiError;
  type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

  fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
    let request = req.clone();
    Box::pin(async move {
      let role = match request.headers().get("auth-user-role") {
        Some(id) => id.to_str(),
        None => return Err(ApiError::new(StatusCode::FORBIDDEN, "forbidden".to_owned())),
      }
      .unwrap();
      if role != "user" && role != "admin" {
        return Err(ApiError::new(StatusCode::FORBIDDEN, "forbidden".to_owned()));
      }
      let id = match request.headers().get("auth-user-id") {
        Some(id) => id.to_str(),
        None => return Err(ApiError::new(StatusCode::FORBIDDEN, "forbidden".to_owned())),
      }
      .unwrap();
      let mail_address = match request.headers().get("auth-user-email") {
        Some(id) => id.to_str(),
        None => return Err(ApiError::new(StatusCode::FORBIDDEN, "forbidden".to_owned())),
      }
      .unwrap();
      Ok(AuthUser {
        id: id.to_string(),
        mail_address: mail_address.to_string(),
        role: role.to_string(),
      })
    })
  }

  fn extract(req: &HttpRequest) -> Self::Future {
    Self::from_request(req, &mut Payload::None)
  }
}

#[derive(Clone)]
pub struct RequestContext {
  pool: Pool<ConnectionManager<PgConnection>>,
}

impl RequestContext {
  pub fn get_connection(&self) -> DBConn {
    let mut conn = self.pool.get().expect("Failed to get connection from pool");

    if cfg!(feature = "integration_test") {
      conn
        .begin_test_transaction()
        .expect("Failed to begin test transaction");
    }

    conn
  }
}

impl Default for RequestContext {
  fn default() -> Self {
    if cfg!(feature = "integration_test") {
      from_filename(".env.test").ok();
    } else {
      from_filename(".env").ok();
    }

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
      .build(manager)
      .expect("Failed to create pool.");

    Self { pool }
  }
}
