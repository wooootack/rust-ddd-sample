use crate::modules::{
  common::pagination::PageInfo,
  user::{
    domain::users_repository::UsersRepository,
    usecase::{find_one_user_usecase::UserResult, usecase_error::UsecaseError},
  },
};
use std::cell::RefCell;

pub struct FindAllUsersResult {
  pub items: Vec<UserResult>,
  pub page_info: PageInfo,
}

pub struct FindAllUsersUsecase<R>
where
  R: UsersRepository,
{
  users_repository: RefCell<R>,
}

impl<R: UsersRepository> FindAllUsersUsecase<R> {
  pub fn new(users_repository: RefCell<R>) -> Self {
    Self { users_repository }
  }

  pub fn execute(&self) -> Result<FindAllUsersResult, UsecaseError> {
    let result = self.users_repository.borrow_mut().find_all()?;

    let converted_users = result.items.into_iter().map(UserResult::from).collect();

    Ok(FindAllUsersResult {
      items: converted_users,
      page_info: result.page_info,
    })
  }
}
