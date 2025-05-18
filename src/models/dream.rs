use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool, Row, postgres::PgRow, types::time::OffsetDateTime};
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

    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Dream>, Error> {
        let record = sqlx::query!(
            r#"
            SELECT d.id, d.text, d.interpretation, d.created_at, 
                   array_agg(dt.tag) FILTER (WHERE dt.tag IS NOT NULL) as tags
            FROM dreams d
            LEFT JOIN dream_tags dt ON d.id = dt.dream_id
            WHERE d.id = $1
            GROUP BY d.id
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
                tags: if let Some(tags) = r.tags {
                    if tags.is_empty() { None } else { Some(tags) }
                } else {
                    None
                },
            })),
            None => Ok(None),
        }
    }

    pub async fn list(pool: &PgPool) -> Result<Vec<Dream>, Error> {
        let records = sqlx::query!(
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

        let dreams = records
            .into_iter()
            .map(|r| Dream {
                id: r.id,
                text: r.text,
                interpretation: r.interpretation,
                created_at: offset_to_chrono(r.created_at),
                tags: if let Some(tags) = r.tags {
                    if tags.is_empty() { None } else { Some(tags) }
                } else {
                    None
                },
            })
            .collect();

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

        // Create base query with counting for pagination
        let mut query_builder = String::from(
            "WITH filtered_dreams AS (
                SELECT d.id
                FROM dreams d
                LEFT JOIN dream_tags dt ON d.id = dt.dream_id
                WHERE 1=1
            ",
        );

        // Add tag filter if provided
        if let Some(tag) = tag_filter {
            query_builder.push_str(&format!("AND dt.tag = '{}' ", tag));
        }

        // Add text search if provided
        if let Some(search) = search_text {
            query_builder.push_str(&format!(
                "AND (d.text ILIKE '%{}%' OR d.interpretation ILIKE '%{}%') ",
                search, search
            ));
        }

        // Complete the CTE
        query_builder.push_str(
            "GROUP BY d.id
            )
            SELECT 
                d.id, d.text, d.interpretation, d.created_at,
                array_agg(dt.tag) FILTER (WHERE dt.tag IS NOT NULL) as tags,
                (SELECT COUNT(*) FROM filtered_dreams) as total_count
            FROM dreams d
            INNER JOIN filtered_dreams fd ON d.id = fd.id
            LEFT JOIN dream_tags dt ON d.id = dt.dream_id
            GROUP BY d.id
            ",
        );

        // Add ORDER BY clause
        query_builder.push_str(&format!("ORDER BY {} {} ", order_column, order_direction));

        // Add LIMIT and OFFSET
        query_builder.push_str(&format!("LIMIT {} OFFSET {}", limit, offset));

        // Execute the query
        let records = sqlx::query(&query_builder)
            .map(|row: PgRow| {
                let id: Uuid = row.get("id");
                let text: String = row.get("text");
                let interpretation: String = row.get("interpretation");
                let created_at: OffsetDateTime = row.get("created_at");
                let tags: Option<Vec<String>> = row.get("tags");
                let total_count: i64 = row.get("total_count");

                (
                    Dream {
                        id,
                        text,
                        interpretation,
                        created_at: offset_to_chrono(Some(created_at)),
                        tags: if let Some(tags) = tags {
                            if tags.is_empty() { None } else { Some(tags) }
                        } else {
                            None
                        },
                    },
                    total_count,
                )
            })
            .fetch_all(pool)
            .await?;

        // Extract dreams and total count
        let total = if records.is_empty() {
            0
        } else {
            records[0].1 as u32
        };

        let dreams = records.into_iter().map(|(dream, _)| dream).collect();
        let total_pages = (total as f64 / limit as f64).ceil() as u32;
        let page = (offset / limit) + 1;

        Ok(PaginatedDreams {
            dreams,
            total,
            page,
            limit,
            total_pages,
        })
    }

    pub async fn add_tag(pool: &PgPool, dream_id: Uuid, tag: &str) -> Result<(), Error> {
        sqlx::query!(
            r#"
            INSERT INTO dream_tags (dream_id, tag)
            VALUES ($1, $2)
            ON CONFLICT (dream_id, tag) DO NOTHING
            "#,
            dream_id,
            tag
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn remove_tag(pool: &PgPool, dream_id: Uuid, tag: &str) -> Result<(), Error> {
        sqlx::query!(
            r#"
            DELETE FROM dream_tags
            WHERE dream_id = $1 AND tag = $2
            "#,
            dream_id,
            tag
        )
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

        // Create base query with full-text search
        let mut query_builder = String::from(
            "WITH filtered_dreams AS (
                SELECT d.id
                FROM dreams d
                LEFT JOIN dream_tags dt ON d.id = dt.dream_id
                WHERE d.search_vector @@ to_tsquery('english', $1)
            ",
        );

        // Add tag filter if provided
        if let Some(tag) = tag_filter {
            query_builder.push_str(&format!("AND dt.tag = '{}' ", tag));
        }

        // Complete the CTE
        query_builder.push_str(
            "GROUP BY d.id
            )
            SELECT 
                d.id, d.text, d.interpretation, d.created_at,
                array_agg(dt.tag) FILTER (WHERE dt.tag IS NOT NULL) as tags,
                (SELECT COUNT(*) FROM filtered_dreams) as total_count,
                ts_rank(d.search_vector, to_tsquery('english', $1)) as rank
            FROM dreams d
            INNER JOIN filtered_dreams fd ON d.id = fd.id
            LEFT JOIN dream_tags dt ON d.id = dt.dream_id
            GROUP BY d.id, d.search_vector
            ",
        );

        // Add ORDER BY clause - for search results, we prioritize rank first, then the user's sort preference
        query_builder.push_str(&format!(
            "ORDER BY rank DESC, {} {} ",
            order_column, order_direction
        ));

        // Add LIMIT and OFFSET
        query_builder.push_str(&format!("LIMIT {} OFFSET {}", limit, offset));

        // Format search query for full-text search
        // Replace spaces with & for AND operation in tsquery
        let formatted_query = query.replace(' ', " & ");

        // Execute the query
        let records = sqlx::query(&query_builder)
            .bind(&formatted_query)
            .map(|row: PgRow| {
                let id: Uuid = row.get("id");
                let text: String = row.get("text");
                let interpretation: String = row.get("interpretation");
                let created_at: OffsetDateTime = row.get("created_at");
                let tags: Option<Vec<String>> = row.get("tags");
                let total_count: i64 = row.get("total_count");

                (
                    Dream {
                        id,
                        text,
                        interpretation,
                        created_at: offset_to_chrono(Some(created_at)),
                        tags: if let Some(tags) = tags {
                            if tags.is_empty() { None } else { Some(tags) }
                        } else {
                            None
                        },
                    },
                    total_count,
                )
            })
            .fetch_all(pool)
            .await?;

        // Extract dreams and total count
        let total = if records.is_empty() {
            0
        } else {
            records[0].1 as u32
        };

        let dreams = records.into_iter().map(|(dream, _)| dream).collect();
        let total_pages = (total as f64 / limit as f64).ceil() as u32;
        let page = (offset / limit) + 1;

        Ok(PaginatedDreams {
            dreams,
            total,
            page,
            limit,
            total_pages,
        })
    }
}
