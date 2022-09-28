use crate::modules::user::{
  domain::{
    mail_address_unique_checker::MailAddressUniqueChecker, user::User, user_id::UserId,
    user_name::UserName, users_repository::UsersRepository,
  },
  usecase::{find_one_user_usecase::UserResult, usecase_error::UsecaseError},
};
use std::{cell::RefCell, rc::Rc};

pub struct CreateUserParameter {
  pub first_name: String,
  pub last_name: String,
  pub mail_address: String,
  pub age: i16,
}

pub struct CreateUserUsecase<R>
where
  R: UsersRepository,
{
  users_repository: Rc<RefCell<R>>,
  mail_address_unique_checker: MailAddressUniqueChecker<R>,
}

impl<R: UsersRepository> CreateUserUsecase<R> {
  pub fn new(
    users_repository: Rc<RefCell<R>>,
    mail_address_unique_checker: MailAddressUniqueChecker<R>,
  ) -> Self {
    Self {
      users_repository,
      mail_address_unique_checker,
    }
  }

  pub fn execute(&self, parameter: CreateUserParameter) -> Result<UserResult, UsecaseError> {
    self
      .mail_address_unique_checker
      .execute(&parameter.mail_address)?;

    let user_id = UserId::default();
    let user_name = UserName::new(parameter.first_name, parameter.last_name);
    let user = User::new(user_id, user_name, parameter.mail_address, parameter.age);

    self.users_repository.borrow_mut().register(&user)?;

    Ok(UserResult::from(user))
  }
}
