use super::{domain_error::DomainError, user::User};
use crate::modules::{
  common::pagination::{PageInfo, Paging},
  user::domain::user_id::UserId,
};

pub struct FindUsersResult {
  pub items: Vec<User>,
  pub page_info: PageInfo,
}

pub trait UsersRepository {
  fn find_all(&mut self) -> Result<FindUsersResult, DomainError>;
  fn find(&mut self, paging: &Paging) -> Result<FindUsersResult, DomainError>;
  fn find_by_id(&mut self, id: &UserId) -> Result<Option<User>, DomainError>;
  fn find_by_email(&mut self, mail_address: &str) -> Result<Option<User>, DomainError>;
  fn register(&mut self, user: &User) -> Result<(), DomainError>;
  fn update(&mut self, user: &User) -> Result<(), DomainError>;
  fn delete(&mut self, user: &User) -> Result<i16, DomainError>;
}
