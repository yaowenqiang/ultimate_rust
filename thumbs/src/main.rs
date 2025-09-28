use axum::response::Html;
use axum::routing::get;
use axum::{Extension, Router};
use sqlx::Row;
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    let db_url = std::env::var("DATABASE_URL")?;

    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new()
        .route("/", get(index_page))
        .layer(Extension(pool));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8280")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn test(Extension(pool): Extension<sqlx::Pool<sqlx::Sqlite>>) -> String {
    let result = sqlx::query("SELECT count(id) FROM images")
        .fetch_one(&pool)
        .await
        .unwrap();
    let count = result.get::<i64, _>(0);
    format!("{count} image(s) in the database")
}

async fn index_page() -> Html<String> {
    let path = Path::new("src/index.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}
