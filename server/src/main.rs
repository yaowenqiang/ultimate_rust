mod collector;

use axum::{Extension, Json, Router, extract::Path, routing::get};
use futures::TryStreamExt;
use serde::Serialize;
use sqlx;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    let db_url = std::env::var("DATABASE_URL")?;

    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    let handle = tokio::spawn(collector::data_collector(pool.clone()));

    let app = Router::new()
        .route("/api/all", get(show_all))
        .route("/api/collectors", get(show_collectors))
        .route("/api/collector/{uuid}", get(collector_data))
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    handle.await??;
    Ok(())
}

use sqlx::{FromRow, Pool};

#[derive(FromRow, Debug, Serialize)]
pub struct DataPoint {
    id: i32,
    collector_id: String,
    received: i64,
    total_memory: i64,
    used_memory: i64,
    average_cpu: f32,
}

#[derive(FromRow, Debug, Serialize)]
pub struct Collector {
    id: i32,
    collector_id: String,
    last_seen: i64,
}

async fn show_all(Extension(pool): Extension<Pool<sqlx::Sqlite>>) -> Json<Vec<DataPoint>> {
    // let mut rows = sqlx::query_as::<_, DataPoint>("select * from timeseries").fetch(&pool);
    let rows = sqlx::query_as::<_, DataPoint>("select * from timeseries")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(rows)

    // while let Some(row) = rows.try_next().await.unwrap() {
    //     println!("{:?}", row);
    // }
}
async fn show_collectors(Extension(pool): Extension<Pool<sqlx::Sqlite>>) -> Json<Vec<Collector>> {
    const SQL: &str = "SELECT
    DISTINCT(id) AS id,
    collector_id,
    (SELECT MAX(received) FROM timeseries WHERE collector_id = ts.collector_id) AS last_seen
    FROM timeseries ts";

    Json(
        sqlx::query_as::<_, Collector>(SQL)
            .fetch_all(&pool)
            .await
            .unwrap(),
    )
}

async fn collector_data(
    Extension(pool): Extension<Pool<sqlx::Sqlite>>,
    uuid: Path<String>,
) -> Json<Vec<DataPoint>> {
    let rows = sqlx::query_as::<_, DataPoint>(
        "select * from timeseries where collector_id = ? order by received",
    )
    .bind(uuid.as_str())
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(rows)
}
