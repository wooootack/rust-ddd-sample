use crate::schema::tasks;
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};

#[derive(Queryable, Insertable)]
#[diesel(table_name = tasks)]
pub struct Task {
  pub id: String,
  pub title: String,
  pub body: String,
  pub user_id: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
