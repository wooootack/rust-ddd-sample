use chrono::{DateTime, Utc};

use crate::modules::task::{
  domain::{task::Task, task_id::TaskId, tasks_repository::TasksRepository},
  usecase::usecase_error::UsecaseError,
};
use std::cell::RefCell;

pub struct CreateTaskParameter {
  pub title: String,
  pub body: String,
  pub user_id: String,
}

pub struct TaskResult {
  pub id: String,
  pub title: String,
  pub body: String,
  pub user_id: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<Task> for TaskResult {
  fn from(task: Task) -> Self {
    TaskResult {
      id: task.id.value,
      title: task.title,
      body: task.body,
      user_id: task.user_id,
      created_at: task.created_at,
      updated_at: task.updated_at,
    }
  }
}

pub struct CreateTaskUseCase<R>
where
  R: TasksRepository,
{
  tasks_repository: RefCell<R>,
}

impl<R: TasksRepository> CreateTaskUseCase<R> {
  pub fn new(tasks_repository: RefCell<R>) -> Self {
    Self { tasks_repository }
  }

  pub fn execute(&self, parameter: CreateTaskParameter) -> Result<TaskResult, UsecaseError> {
    let id = TaskId::default();
    let title = parameter.title;
    let body = parameter.body;
    let user_id = parameter.user_id;

    let task = Task::new(id, title, body, user_id);

    self.tasks_repository.borrow_mut().register(&task)?;

    Ok(TaskResult::from(task))
  }
}
