use crate::modules::user::{
  domain::{user_id::UserId, users_repository::UsersRepository},
  usecase::{find_one_user_usecase::UserResult, usecase_error::UsecaseError},
};
use std::cell::RefCell;

pub struct DeleteUserParameter {
  pub id: String,
}

pub struct DeleteUserUsecase<R>
where
  R: UsersRepository,
{
  users_repository: RefCell<R>,
}

impl<R: UsersRepository> DeleteUserUsecase<R> {
  pub fn new(users_repository: RefCell<R>) -> Self {
    Self { users_repository }
  }

  pub fn execute(&self, parameter: DeleteUserParameter) -> Result<UserResult, UsecaseError> {
    let user_id = UserId::restore(parameter.id);

    let maybe_user = self.users_repository.borrow_mut().find_by_id(&user_id)?;

    let user = match maybe_user {
      Some(user) => user,
      None => {
        return Err(UsecaseError::invalid(
          "対象のユーザーが見つかりませんでした。".to_string(),
        ))
      }
    };

    self.users_repository.borrow_mut().delete(&user)?;

    Ok(UserResult::from(user))
  }
}
