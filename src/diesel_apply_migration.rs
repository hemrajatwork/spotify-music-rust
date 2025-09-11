use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
use std::error::Error;
use crate::db_lib::establish_connection;
use std::marker::Send;

pub fn run_migrations() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {

    let mut connection = establish_connection();
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}