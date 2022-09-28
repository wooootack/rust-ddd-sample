use crate::modules::user::{
  domain::{user::User, users_repository::UsersRepository},
  usecase::usecase_error::UsecaseError,
};
use chrono::{DateTime, Utc};
use std::cell::RefCell;

pub struct FindOneUserParameter {
  pub id: String,
}

pub struct UserResult {
  pub id: String,
  pub first_name: String,
  pub last_name: String,
  pub mail_address: String,
  pub age: i16,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResult {
  fn from(user: User) -> Self {
    UserResult {
      id: user.id.value,
      first_name: user.name.first_name,
      last_name: user.name.last_name,
      mail_address: user.mail_address,
      age: user.age,
      created_at: user.created_at,
      updated_at: user.updated_at,
    }
  }
}

pub struct FindOneUserUsecase<R>
where
  R: UsersRepository,
{
  users_repository: RefCell<R>,
}

impl<R: UsersRepository> FindOneUserUsecase<R> {
  pub fn new(users_repository: RefCell<R>) -> Self {
    Self { users_repository }
  }

  pub fn execute(
    &self,
    parameter: FindOneUserParameter,
  ) -> Result<Option<UserResult>, UsecaseError> {
    let user_id = match parameter.id.try_into() {
      Ok(id) => id,
      Err(_) => return Ok(None),
    };
    let maybe_user = self.users_repository.borrow_mut().find_by_id(&user_id)?;

    Ok(maybe_user.map(UserResult::from))
  }
}
