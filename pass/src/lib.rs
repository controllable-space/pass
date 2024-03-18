pub mod repositories;
pub mod models;
pub mod schema;

#[cfg(test)]
pub mod tests {
    use diesel::prelude::*;
    use diesel_async::{AsyncConnection, AsyncPgConnection};
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    pub async fn database_connection() -> anyhow::Result<AsyncPgConnection> {
        let database_url = std::env::var("TEST_DATABASE_URL")?;
        let mut sync_conn = PgConnection::establish(&database_url)?;
        sync_conn.run_pending_migrations(MIGRATIONS).expect("Failed to run pending migrations");

        let conn = AsyncPgConnection::establish(&database_url).await?;
        Ok(conn)
    }
}
