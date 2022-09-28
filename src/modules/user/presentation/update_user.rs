use crate::{
  modules::{
    common::api_error::ApiError,
    user::{
      infra::postgres_users_repository::PostgresUsersRepository,
      presentation::find_one_user::UserResponse,
      usecase::update_user_usecase::{UpdateUserParameter, UpdateUserUsecase},
    },
  },
  scenarios::request_context::DBConn,
};
use std::{cell::RefCell, rc::Rc};

pub struct UpdateUserRequest {
  pub id: String,
  pub first_name: String,
  pub last_name: String,
  pub age: i16,
}

pub fn execute(conn: DBConn, request: UpdateUserRequest) -> Result<UserResponse, ApiError> {
  let conn = Rc::new(RefCell::new(conn));
  let tasks_repository = RefCell::new(PostgresUsersRepository::new(conn));
  let usecase = UpdateUserUsecase::new(tasks_repository);
  let parameter = UpdateUserParameter {
    id: request.id,
    first_name: request.first_name,
    last_name: request.last_name,
    age: request.age,
  };
  let result = usecase.execute(parameter)?;

  Ok(UserResponse::from(result))
}
