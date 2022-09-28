use crate::{
  modules::{
    diesel::tasks::Task,
    task::domain::{
      domain_error::DomainError, task::Task as DomainTask, tasks_repository::TasksRepository,
    },
  },
  scenarios::request_context::DBConn,
  schema::tasks::dsl::*,
};
use diesel::prelude::*;
use std::{cell::RefCell, ops::DerefMut, rc::Rc};

pub struct PostgresTasksRepository {
  conn: Rc<RefCell<DBConn>>,
}

impl PostgresTasksRepository {
  pub fn new(conn: Rc<RefCell<DBConn>>) -> Self {
    Self { conn }
  }
}

impl TasksRepository for PostgresTasksRepository {
  fn register(&mut self, task: &DomainTask) -> Result<(), DomainError> {
    diesel::insert_into(tasks)
      .values(Task::from(task.clone()))
      .execute(self.conn.borrow_mut().deref_mut())?;

    Ok(())
  }
}
