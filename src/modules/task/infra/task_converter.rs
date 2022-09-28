use crate::modules::{diesel::tasks::Task, task::domain::task::Task as DomainTask};

impl From<DomainTask> for Task {
  fn from(domain_task: DomainTask) -> Self {
    Task {
      id: domain_task.id.value,
      title: domain_task.title,
      body: domain_task.body,
      user_id: domain_task.user_id,
      created_at: domain_task.created_at,
      updated_at: domain_task.updated_at,
    }
  }
}
