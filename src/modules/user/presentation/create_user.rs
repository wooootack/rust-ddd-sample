use crate::{
  modules::{
    common::api_error::ApiError,
    user::{
      domain::mail_address_unique_checker::MailAddressUniqueChecker,
      infra::postgres_users_repository::PostgresUsersRepository,
      presentation::find_one_user::UserResponse,
      usecase::create_user_usecase::{CreateUserParameter, CreateUserUsecase},
    },
  },
  scenarios::request_context::DBConn,
};
use std::{cell::RefCell, rc::Rc};

pub struct CreateUserRequest {
  pub first_name: String,
  pub last_name: String,
  pub mail_address: String,
  pub age: i16,
}

pub fn execute(conn: DBConn, request: CreateUserRequest) -> Result<UserResponse, ApiError> {
  let conn = Rc::new(RefCell::new(conn));
  let tasks_repository = Rc::new(RefCell::new(PostgresUsersRepository::new(conn)));
  let mail_address_unique_checker = MailAddressUniqueChecker::new(tasks_repository.clone());
  let usecase = CreateUserUsecase::new(tasks_repository, mail_address_unique_checker);
  let parameter = CreateUserParameter {
    first_name: request.first_name,
    last_name: request.last_name,
    mail_address: request.mail_address,
    age: request.age,
  };
  let result = usecase.execute(parameter)?;

  Ok(UserResponse::from(result))
}
