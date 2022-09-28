use super::{domain_error::DomainError, task::Task};

pub trait TasksRepository {
  fn register(&mut self, user: &Task) -> Result<(), DomainError>;
}
