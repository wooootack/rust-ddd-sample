use crate::{
  modules::{
    common::api_error::ApiError,
    user::{
      infra::postgres_users_repository::PostgresUsersRepository,
      usecase::delete_user_usecase::{DeleteUserParameter, DeleteUserUsecase},
    },
  },
  scenarios::request_context::DBConn,
};
use std::{cell::RefCell, rc::Rc};

pub struct DeleteUserRequest {
  pub id: String,
}

pub fn execute(conn: DBConn, request: DeleteUserRequest) -> Result<(), ApiError> {
  let conn = Rc::new(RefCell::new(conn));
  let tasks_repository = RefCell::new(PostgresUsersRepository::new(conn));
  let usecase = DeleteUserUsecase::new(tasks_repository);
  let parameter = DeleteUserParameter { id: request.id };
  usecase.execute(parameter)?;

  Ok(())
}
