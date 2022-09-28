use crate::{
  modules::{
    common::api_error::ApiError,
    user::{
      infra::postgres_users_repository::PostgresUsersRepository,
      usecase::find_one_user_usecase::{FindOneUserParameter, FindOneUserUsecase, UserResult},
    },
  },
  scenarios::request_context::DBConn,
};
use chrono::{DateTime, Utc};
use std::{cell::RefCell, rc::Rc};

pub struct FindOneUserRequest {
  pub id: String,
}

pub struct UserResponse {
  pub id: String,
  pub first_name: String,
  pub last_name: String,
  pub mail_address: String,
  pub age: i16,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<UserResult> for UserResponse {
  fn from(result: UserResult) -> Self {
    UserResponse {
      id: result.id,
      first_name: result.first_name,
      last_name: result.last_name,
      mail_address: result.mail_address,
      age: result.age,
      created_at: result.created_at,
      updated_at: result.updated_at,
    }
  }
}

pub fn execute(
  conn: DBConn,
  request: FindOneUserRequest,
) -> Result<Option<UserResponse>, ApiError> {
  let conn = Rc::new(RefCell::new(conn));
  let tasks_repository = RefCell::new(PostgresUsersRepository::new(conn));
  let usecase = FindOneUserUsecase::new(tasks_repository);
  let parameter = FindOneUserParameter { id: request.id };
  let result = usecase.execute(parameter)?;

  match result {
    Some(a) => Ok(Some(UserResponse::from(a))),
    None => Ok(None),
  }
}
