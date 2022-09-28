use crate::modules::user::domain::{domain_error::DomainError, users_repository::UsersRepository};
use std::{cell::RefCell, rc::Rc};

pub struct MailAddressUniqueChecker<R>
where
  R: UsersRepository,
{
  users_repository: Rc<RefCell<R>>,
}

impl<R: UsersRepository> MailAddressUniqueChecker<R> {
  pub fn new(users_repository: Rc<RefCell<R>>) -> Self {
    Self { users_repository }
  }

  pub fn execute(&self, mail_address: &str) -> Result<(), DomainError> {
    let user = self
      .users_repository
      .borrow_mut()
      .find_by_email(mail_address)?;

    match user {
      Some(_) => Err(DomainError::invalid(
        "メールアドレスは既に登録されています".to_string(),
      )),
      None => Ok(()),
    }
  }
}
