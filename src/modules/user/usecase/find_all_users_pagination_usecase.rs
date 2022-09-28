use crate::modules::{
  common::pagination::Paging,
  user::{
    domain::users_repository::UsersRepository,
    usecase::{
      find_all_users_usecase::FindAllUsersResult, find_one_user_usecase::UserResult,
      usecase_error::UsecaseError,
    },
  },
};
use std::cell::RefCell;

pub struct FindAllUsersPaginationUsecase<R>
where
  R: UsersRepository,
{
  users_repository: RefCell<R>,
}

impl<R: UsersRepository> FindAllUsersPaginationUsecase<R> {
  pub fn new(users_repository: RefCell<R>) -> Self {
    Self { users_repository }
  }

  pub fn execute(&self, paging: &Paging) -> Result<FindAllUsersResult, UsecaseError> {
    let result = self.users_repository.borrow_mut().find(paging)?;

    let converted_users = result.items.into_iter().map(UserResult::from).collect();

    Ok(FindAllUsersResult {
      items: converted_users,
      page_info: result.page_info,
    })
  }
}
