use chrono::{DateTime, Utc};

use super::task_id::TaskId;

#[derive(Clone)]
pub struct Task {
  pub id: TaskId,
  pub title: String,
  pub body: String,
  pub user_id: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl Task {
  pub fn new(id: TaskId, title: String, body: String, user_id: String) -> Self {
    let now = Utc::now();

    Self {
      id,
      title,
      body,
      user_id,
      created_at: now,
      updated_at: now,
    }
  }

  pub fn restore(
    id: TaskId,
    title: String,
    body: String,
    user_id: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
  ) -> Self {
    Self {
      id,
      title,
      body,
      user_id,
      created_at,
      updated_at,
    }
  }
}
