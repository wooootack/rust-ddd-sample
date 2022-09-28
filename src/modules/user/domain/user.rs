use chrono::{DateTime, Utc};

use super::{user_id::UserId, user_name::UserName};

#[derive(Clone)]
pub struct User {
  pub id: UserId,
  pub name: UserName,
  pub mail_address: String,
  pub age: i16,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl User {
  pub fn new(id: UserId, name: UserName, mail_address: String, age: i16) -> Self {
    let now = Utc::now();

    Self {
      id,
      name,
      mail_address,
      age,
      created_at: now,
      updated_at: now,
    }
  }

  pub fn restore(
    id: UserId,
    name: UserName,
    mail_address: String,
    age: i16,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
  ) -> Self {
    Self {
      id,
      name,
      mail_address,
      age,
      created_at,
      updated_at,
    }
  }

  pub fn change_name(&mut self, user_name: UserName) {
    self.name = user_name;
    self.update_updated_at();
  }

  pub fn change_age(&mut self, age: i16) {
    self.age = age;
    self.update_updated_at();
  }

  fn update_updated_at(&mut self) {
    self.updated_at = Utc::now();
  }
}
