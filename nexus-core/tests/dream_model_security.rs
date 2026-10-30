use axum_server::models::dream::{Dream, SortField, SortOrder};

#[tokio::test]
#[ignore = "requires a local PostgreSQL database with Nexus Core migrations"]
async fn tag_filter_treats_quotes_as_data() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let pool = sqlx::PgPool::connect(&database_url).await.unwrap();
    let id = Dream::create(&pool, "A test dream", "A test interpretation")
        .await
        .unwrap();
    let hostile_tag = format!("x{}' OR 1=1 --", uuid::Uuid::new_v4().simple());

    let result = Dream::list_paginated(
        &pool,
        0,
        10,
        SortField::CreatedAt,
        SortOrder::Descending,
        Some(&hostile_tag),
        None,
    )
    .await;

    sqlx::query("DELETE FROM dreams WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();

    let page = result.expect("tag text must not change SQL syntax");
    assert!(!page.dreams.iter().any(|dream| dream.id == id));
}

#[tokio::test]
#[ignore = "requires a local PostgreSQL database with Nexus Core migrations"]
async fn full_text_search_accepts_punctuation() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let pool = sqlx::PgPool::connect(&database_url).await.unwrap();
    let id = Dream::create(&pool, "The moon and ocean were bright", "A reflection")
        .await
        .unwrap();

    let result = Dream::search(
        &pool,
        "moon & ocean",
        0,
        10,
        SortField::CreatedAt,
        SortOrder::Descending,
        None,
    )
    .await;

    sqlx::query("DELETE FROM dreams WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();

    let page = result.expect("punctuation must be treated as search text");
    assert!(page.total >= 1);
    assert!(page.dreams.iter().any(|dream| dream.id == id));
}
