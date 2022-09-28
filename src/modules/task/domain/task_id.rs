use serde::Serialize;
use ulid::Ulid;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct TaskId {
  pub value: String,
}

impl Default for TaskId {
  fn default() -> Self {
    let value = Ulid::new().to_string();
    Self { value }
  }
}

impl TaskId {
  pub fn restore(value: String) -> Self {
    let value = Ulid::from_string(&value).unwrap().to_string();
    Self { value }
  }
}
