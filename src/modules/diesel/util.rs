use diesel::PgConnection;
use diesel_migrations::{FileBasedMigrations, MigrationHarness};
use std::error::Error;

type CustomError = Box<dyn Error + Send + Sync>;

pub fn has_pending_migration(conn: &mut PgConnection, path: &str) -> Result<bool, CustomError> {
  let dir = FileBasedMigrations::from_path(path)?;
  MigrationHarness::has_pending_migration(conn, dir)
}
