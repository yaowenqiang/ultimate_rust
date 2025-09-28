use axum::routing::get;
use axum::{Extension, Router};
use sqlx::Row;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    let db_url = std::env::var("DATABASE_URL")?;

    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new().route("/", get(test)).layer(Extension(pool));
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
