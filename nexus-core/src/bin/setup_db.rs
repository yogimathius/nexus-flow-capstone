use clap::{ArgAction, Parser};
use std::env;
use std::error::Error;

#[derive(Parser, Debug)]
#[clap(author, version, about = "Setup the Dream API database")]
struct Args {
    #[clap(short, long, action=ArgAction::SetTrue)]
    reset: bool,

    #[clap(long)]
    database_url: Option<String>,
}

async fn execute_sql(db_url: &str, sql: &str, description: &str) -> Result<(), Box<dyn Error>> {
    println!("{}...", description);

    let (client, connection) = tokio_postgres::connect(db_url, tokio_postgres::NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    match client.simple_query(sql).await {
        Ok(_) => println!("  ✅ {}", description),
        Err(e) => {
            println!("  ❌ Error {}: {}", description, e);
            return Err(Box::new(e));
        }
    }

    Ok(())
}

async fn drop_tables(db_url: &str) -> Result<(), Box<dyn Error>> {
    println!("\n🗑️  Dropping existing tables...");

    let drop_statements = [
        (
            "DROP INDEX IF EXISTS idx_dreams_search",
            "Dropping search index",
        ),
        (
            "DROP INDEX IF EXISTS idx_dream_tags_tag",
            "Dropping tag index",
        ),
        (
            "DROP TABLE IF EXISTS dream_tags",
            "Dropping dream_tags table",
        ),
        ("DROP TABLE IF EXISTS dreams", "Dropping dreams table"),
    ];

    for (statement, description) in drop_statements {
        execute_sql(db_url, statement, description).await?;
    }

    println!("✅ All tables dropped successfully\n");
    Ok(())
}

async fn create_tables(db_url: &str) -> Result<(), Box<dyn Error>> {
    println!("🔨 Creating database schema...");

    // Create extensions
    execute_sql(
        db_url,
        "CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"",
        "Creating uuid-ossp extension",
    )
    .await?;

    // Create dreams table
    let dreams_table_sql = r#"
        CREATE TABLE IF NOT EXISTS dreams (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            text TEXT NOT NULL,
            interpretation TEXT NOT NULL,
            created_at TIMESTAMPTZ DEFAULT now()
        )
    "#;

    execute_sql(db_url, dreams_table_sql, "Creating dreams table").await?;

    // Create dream_tags table
    let dream_tags_table_sql = r#"
        CREATE TABLE IF NOT EXISTS dream_tags (
            dream_id UUID NOT NULL REFERENCES dreams(id) ON DELETE CASCADE,
            tag TEXT NOT NULL,
            created_at TIMESTAMPTZ DEFAULT now(),
            PRIMARY KEY (dream_id, tag)
        )
    "#;

    execute_sql(db_url, dream_tags_table_sql, "Creating dream_tags table").await?;

    // Create indexes
    execute_sql(
        db_url,
        "CREATE INDEX IF NOT EXISTS idx_dream_tags_tag ON dream_tags(tag)",
        "Creating tag index",
    )
    .await?;

    // Add search vector column to dreams table
    execute_sql(
        db_url,
        "ALTER TABLE dreams ADD COLUMN IF NOT EXISTS search_vector tsvector GENERATED ALWAYS AS (to_tsvector('english', text || ' ' || interpretation)) STORED",
        "Adding search vector column",
    ).await?;

    // Create search index
    execute_sql(
        db_url,
        "CREATE INDEX IF NOT EXISTS idx_dreams_search ON dreams USING GIN(search_vector)",
        "Creating search index",
    )
    .await?;

    println!("✅ Schema creation complete\n");
    Ok(())
}

async fn get_table_counts(db_url: &str) -> Result<(), Box<dyn Error>> {
    println!("📊 Checking table status...");

    let (client, connection) = tokio_postgres::connect(db_url, tokio_postgres::NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Check dreams table
    let result = client.simple_query("SELECT COUNT(*) FROM dreams").await;
    match result {
        Ok(rows) => {
            if let Some(tokio_postgres::SimpleQueryMessage::Row(row)) = rows.first() {
                if let Some(count_str) = row.get(0) {
                    println!("  Dreams: {} records", count_str);
                }
            }
        }
        Err(e) => println!("  Dreams: Error counting - {}", e),
    }

    // Check dream_tags table
    let result = client.simple_query("SELECT COUNT(*) FROM dream_tags").await;
    match result {
        Ok(rows) => {
            if let Some(tokio_postgres::SimpleQueryMessage::Row(row)) = rows.first() {
                if let Some(count_str) = row.get(0) {
                    println!("  Dream Tags: {} records", count_str);
                }
            }
        }
        Err(e) => println!("  Dream Tags: Error counting - {}", e),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Get database URL from command line or environment
    let database_url = args.database_url
        .or_else(|| env::var("DATABASE_URL").ok())
        .expect("DATABASE_URL must be provided either as --database-url argument or DATABASE_URL environment variable");

    println!("🚀 Dream API Database Setup");
    println!("Connecting to database...");

    if args.reset {
        drop_tables(&database_url).await?;
    }

    create_tables(&database_url).await?;

    get_table_counts(&database_url).await?;

    println!("🎉 Database setup complete!");

    Ok(())
}
