use crate::modules::user::{
  domain::{user_id::UserId, user_name::UserName, users_repository::UsersRepository},
  usecase::{find_one_user_usecase::UserResult, usecase_error::UsecaseError},
};
use std::cell::RefCell;

pub struct UpdateUserParameter {
  pub id: String,
  pub first_name: String,
  pub last_name: String,
  pub age: i16,
}

pub struct UpdateUserUsecase<R>
where
  R: UsersRepository,
{
  users_repository: RefCell<R>,
}

impl<R: UsersRepository> UpdateUserUsecase<R> {
  pub fn new(users_repository: RefCell<R>) -> Self {
    Self { users_repository }
  }

  pub fn execute(&self, parameter: UpdateUserParameter) -> Result<UserResult, UsecaseError> {
    let user_id = UserId::restore(parameter.id);

    let maybe_user = self.users_repository.borrow_mut().find_by_id(&user_id)?;

    let mut user = match maybe_user {
      Some(user) => user,
      None => {
        return Err(UsecaseError::invalid(
          "対象のユーザーが見つかりませんでした。".to_string(),
        ))
      }
    };

    let user_name = UserName::new(parameter.first_name, parameter.last_name);

    user.change_name(user_name);
    user.change_age(parameter.age);

    self.users_repository.borrow_mut().update(&user)?;

    Ok(UserResult::from(user))
  }
}
