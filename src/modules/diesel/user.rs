use crate::schema::users;
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};

#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
  pub id: String,
  pub first_name: String,
  pub last_name: String,
  pub mail_address: String,
  pub age: i16,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
