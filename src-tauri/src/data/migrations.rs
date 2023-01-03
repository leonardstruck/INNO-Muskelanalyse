use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn run_migrations(connection: &mut diesel::sqlite::SqliteConnection) {
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}
