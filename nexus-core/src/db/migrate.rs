use sqlx::{PgPool, migrate::MigrateDatabase, postgres::Postgres};
use std::path::Path;

// Run database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    println!("Running database migrations...");

    let migrations = sqlx::migrate::Migrator::new(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/migrations"))).await?;

    migrations.run(pool).await?;

    println!("Migrations complete!");

    Ok(())
}

// Create the database if it doesn't exist
pub async fn create_database_if_not_exists(database_url: &str) -> Result<(), sqlx::Error> {
    let db_exists = Postgres::database_exists(database_url).await?;

    if !db_exists {
        println!("Database does not exist, creating...");
        Postgres::create_database(database_url).await?;
        println!("Database created successfully");
    }

    Ok(())
}
