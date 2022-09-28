use chrono::{DateTime, Utc};

use crate::{
  modules::{
    common::api_error::ApiError,
    task::{
      infra::postgres_tasks_repository::PostgresTasksRepository,
      usecase::create_task_usecase::{CreateTaskParameter, CreateTaskUseCase, TaskResult},
    },
  },
  scenarios::request_context::DBConn,
};
use std::{cell::RefCell, rc::Rc};

pub struct CreateTaskRequest {
  pub title: String,
  pub body: String,
  pub user_id: String,
}

pub struct TaskResponse {
  pub id: String,
  pub title: String,
  pub body: String,
  pub user_id: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<TaskResult> for TaskResponse {
  fn from(task: TaskResult) -> Self {
    TaskResponse {
      id: task.id,
      title: task.title,
      body: task.body,
      user_id: task.user_id,
      created_at: task.created_at,
      updated_at: task.updated_at,
    }
  }
}

pub fn execute(conn: DBConn, request: CreateTaskRequest) -> Result<TaskResponse, ApiError> {
  let conn = Rc::new(RefCell::new(conn));
  let tasks_repository = RefCell::new(PostgresTasksRepository::new(conn));
  let usecase = CreateTaskUseCase::new(tasks_repository);
  let parameter = CreateTaskParameter {
    title: request.title,
    body: request.body,
    user_id: request.user_id,
  };
  let result = usecase.execute(parameter)?;

  Ok(TaskResponse::from(result))
}
