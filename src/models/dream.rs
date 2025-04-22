use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool, types::time::OffsetDateTime};
use uuid::Uuid;

pub type DreamId = Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dream {
    pub id: Uuid,
    pub text: String,
    pub interpretation: String,
    pub created_at: DateTime<Utc>,
}

// Helper function to convert OffsetDateTime to DateTime<Utc>
fn offset_to_chrono(dt: Option<OffsetDateTime>) -> DateTime<Utc> {
    match dt {
        Some(dt) => {
            let timestamp = dt.unix_timestamp();
            let nsec = dt.nanosecond();
            Utc.timestamp_opt(timestamp, nsec).unwrap()
        }
        None => Utc::now(),
    }
}

impl Dream {
    // Create a new dream in the database
    pub async fn create(pool: &PgPool, text: &str, interpretation: &str) -> Result<Uuid, Error> {
        let record = sqlx::query!(
            r#"
            INSERT INTO dreams (text, interpretation)
            VALUES ($1, $2)
            RETURNING id
            "#,
            text,
            interpretation
        )
        .fetch_one(pool)
        .await?;

        Ok(record.id)
    }

    // Get a dream by ID
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Dream>, Error> {
        let record = sqlx::query!(
            r#"
            SELECT id, text, interpretation, created_at
            FROM dreams
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        match record {
            Some(r) => Ok(Some(Dream {
                id: r.id,
                text: r.text,
                interpretation: r.interpretation,
                created_at: offset_to_chrono(r.created_at),
            })),
            None => Ok(None),
        }
    }

    // List all dreams in reverse chronological order
    pub async fn list(pool: &PgPool) -> Result<Vec<Dream>, Error> {
        let records = sqlx::query!(
            r#"
            SELECT id, text, interpretation, created_at
            FROM dreams
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        // Convert database records to our Dream struct
        let dreams = records
            .into_iter()
            .map(|r| Dream {
                id: r.id,
                text: r.text,
                interpretation: r.interpretation,
                created_at: offset_to_chrono(r.created_at),
            })
            .collect();

        Ok(dreams)
    }
}
