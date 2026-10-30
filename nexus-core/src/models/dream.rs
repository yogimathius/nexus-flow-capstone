use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool, types::time::OffsetDateTime};
use uuid::Uuid;

pub type DreamId = Uuid;

// Response type for paginated dreams
#[derive(Debug, Serialize)]
pub struct PaginatedDreams {
    pub dreams: Vec<Dream>,
    pub total: u32,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

// Enum for defining sort order
#[derive(Debug, Deserialize, Clone, Copy)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

impl Default for SortOrder {
    fn default() -> Self {
        SortOrder::Descending
    }
}

// Enum for sorting fields
#[derive(Debug, Deserialize, Clone, Copy)]
pub enum SortField {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "text")]
    Text,
}

impl Default for SortField {
    fn default() -> Self {
        SortField::CreatedAt
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dream {
    #[serde(serialize_with = "serialize_uuid")]
    pub id: Uuid,
    pub text: String,
    pub interpretation: String,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(sqlx::FromRow)]
struct DreamRecord {
    id: Uuid,
    text: String,
    interpretation: String,
    created_at: Option<OffsetDateTime>,
    tags: Option<Vec<String>>,
}

impl From<DreamRecord> for Dream {
    fn from(record: DreamRecord) -> Self {
        Self {
            id: record.id,
            text: record.text,
            interpretation: record.interpretation,
            created_at: offset_to_chrono(record.created_at),
            tags: record.tags.filter(|tags| !tags.is_empty()),
        }
    }
}

// Helper function to serialize UUID as String
fn serialize_uuid<S>(id: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&id.to_string())
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
    pub async fn create(pool: &PgPool, text: &str, interpretation: &str) -> Result<Uuid, Error> {
        let id = sqlx::query_scalar::<_, Uuid>(
            "INSERT INTO dreams (text, interpretation) VALUES ($1, $2) RETURNING id",
        )
        .bind(text)
        .bind(interpretation)
        .fetch_one(pool)
        .await?;

        Ok(id)
    }

    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Dream>, Error> {
        let record = sqlx::query_as::<_, DreamRecord>(
            r#"
            SELECT d.id, d.text, d.interpretation, d.created_at, 
                   array_agg(dt.tag) FILTER (WHERE dt.tag IS NOT NULL) as tags
            FROM dreams d
            LEFT JOIN dream_tags dt ON d.id = dt.dream_id
            WHERE d.id = $1
            GROUP BY d.id
            "#
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(record.map(Dream::from))
    }

    pub async fn list(pool: &PgPool) -> Result<Vec<Dream>, Error> {
        let records = sqlx::query_as::<_, DreamRecord>(
            r#"
            SELECT d.id, d.text, d.interpretation, d.created_at,
                   array_agg(dt.tag) FILTER (WHERE dt.tag IS NOT NULL) as tags
            FROM dreams d
            LEFT JOIN dream_tags dt ON d.id = dt.dream_id
            GROUP BY d.id
            ORDER BY d.created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        let dreams = records.into_iter().map(Dream::from).collect();

        Ok(dreams)
    }

    pub async fn list_paginated(
        pool: &PgPool,
        offset: u32,
        limit: u32,
        sort_field: SortField,
        sort_order: SortOrder,
        tag_filter: Option<&str>,
        search_text: Option<&str>,
    ) -> Result<PaginatedDreams, Error> {
        // Build the query dynamically based on sort and filter parameters
        let order_direction = match sort_order {
            SortOrder::Ascending => "ASC",
            SortOrder::Descending => "DESC",
        };

        let order_column = match sort_field {
            SortField::CreatedAt => "d.created_at",
            SortField::Text => "d.text",
        };

        let limit = limit.max(1);
        const FILTER: &str = "($1::text IS NULL OR EXISTS (SELECT 1 FROM dream_tags filter_tag WHERE filter_tag.dream_id = d.id AND filter_tag.tag = $1)) AND ($2::text IS NULL OR d.text ILIKE '%' || $2 || '%' OR d.interpretation ILIKE '%' || $2 || '%')";
        let total: i64 = sqlx::query_scalar(&format!("SELECT COUNT(*) FROM dreams d WHERE {FILTER}"))
            .bind(tag_filter)
            .bind(search_text)
            .fetch_one(pool)
            .await?;
        let query = format!(
            "SELECT d.id, d.text, d.interpretation, d.created_at, array_agg(dt.tag) FILTER (WHERE dt.tag IS NOT NULL) AS tags \
             FROM dreams d LEFT JOIN dream_tags dt ON d.id = dt.dream_id \
             WHERE {FILTER} GROUP BY d.id ORDER BY {order_column} {order_direction} LIMIT $3 OFFSET $4"
        );
        let records = sqlx::query_as::<_, DreamRecord>(&query)
            .bind(tag_filter)
            .bind(search_text)
            .bind(i64::from(limit))
            .bind(i64::from(offset))
            .fetch_all(pool)
            .await?;

        let total = total.max(0).min(u32::MAX as i64) as u32;
        let dreams = records.into_iter().map(Dream::from).collect();
        let total_pages = total.div_ceil(limit);
        let page = (offset / limit).saturating_add(1);

        Ok(PaginatedDreams {
            dreams,
            total,
            page,
            limit,
            total_pages,
        })
    }

    pub async fn add_tag(pool: &PgPool, dream_id: Uuid, tag: &str) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO dream_tags (dream_id, tag)
            VALUES ($1, $2)
            ON CONFLICT (dream_id, tag) DO NOTHING
            "#
        )
        .bind(dream_id)
        .bind(tag)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn remove_tag(pool: &PgPool, dream_id: Uuid, tag: &str) -> Result<(), Error> {
        sqlx::query(
            r#"
            DELETE FROM dream_tags
            WHERE dream_id = $1 AND tag = $2
            "#
        )
        .bind(dream_id)
        .bind(tag)
        .execute(pool)
        .await?;

        Ok(())
    }

    // Specialized search method that leverages the search_vector column for more efficient text searching
    pub async fn search(
        pool: &PgPool,
        query: &str,
        offset: u32,
        limit: u32,
        sort_field: SortField,
        sort_order: SortOrder,
        tag_filter: Option<&str>,
    ) -> Result<PaginatedDreams, Error> {
        // Build the query dynamically based on sort and filter parameters
        let order_direction = match sort_order {
            SortOrder::Ascending => "ASC",
            SortOrder::Descending => "DESC",
        };

        let order_column = match sort_field {
            SortField::CreatedAt => "d.created_at",
            SortField::Text => "d.text",
        };

        let limit = limit.max(1);
        const FILTER: &str = "d.search_vector @@ plainto_tsquery('english', $1) AND ($2::text IS NULL OR EXISTS (SELECT 1 FROM dream_tags filter_tag WHERE filter_tag.dream_id = d.id AND filter_tag.tag = $2))";
        let total: i64 = sqlx::query_scalar(&format!("SELECT COUNT(*) FROM dreams d WHERE {FILTER}"))
            .bind(query)
            .bind(tag_filter)
            .fetch_one(pool)
            .await?;
        let statement = format!(
            "SELECT d.id, d.text, d.interpretation, d.created_at, array_agg(dt.tag) FILTER (WHERE dt.tag IS NOT NULL) AS tags \
             FROM dreams d LEFT JOIN dream_tags dt ON d.id = dt.dream_id \
             WHERE {FILTER} GROUP BY d.id, d.search_vector \
             ORDER BY ts_rank(d.search_vector, plainto_tsquery('english', $1)) DESC, {order_column} {order_direction} LIMIT $3 OFFSET $4"
        );
        let records = sqlx::query_as::<_, DreamRecord>(&statement)
            .bind(query)
            .bind(tag_filter)
            .bind(i64::from(limit))
            .bind(i64::from(offset))
            .fetch_all(pool)
            .await?;

        let total = total.max(0).min(u32::MAX as i64) as u32;
        let dreams = records.into_iter().map(Dream::from).collect();
        let total_pages = total.div_ceil(limit);
        let page = (offset / limit).saturating_add(1);

        Ok(PaginatedDreams {
            dreams,
            total,
            page,
            limit,
            total_pages,
        })
    }
}
