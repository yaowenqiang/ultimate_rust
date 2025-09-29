use axum::body::StreamBody;
use axum::extract::Multipart;
use axum::http::{HeaderMap, header};
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::{Extension, Router};
use serde::de::Unexpected::Option;
use sqlx::sqlite::SqliteRow;
use sqlx::{Row, Sqlite};
use std::path::Path;
use tokio_util::io::ReaderStream;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    let db_url = std::env::var("DATABASE_URL")?;

    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new()
        .route("/", get(index_page))
        .route("/upload", post(uploader))
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
    let path = std::path::Path::new("src/index.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}

async fn uploader(
    Extension(pool): Extension<sqlx::Pool<sqlx::Sqlite>>,
    mut multipart: Multipart,
) -> String {
    // while let Some(mut field) = multipart.next_field().await.unwrap() {
    //     let name = field.name().unwrap().to_string();
    //     let data = field.bytes().await.unwrap();
    //     println!("name {name} in {} bytes", data.len());
    // }
    let mut tags = None;
    let mut image = None;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        match name.as_str() {
            "tags" => {
                tags = Some(String::from_utf8(data.to_vec()).unwrap());
            }
            "image" => {
                image = Some(data.to_vec());
            }
            _ => {
                panic!("unknown field {name}");
            }
        }
    }
    if let (Some(tags), Some(image)) = (tags, image) {
        let new_image_id = insert_image_into_database(&pool, &tags).await.unwrap();
        save_image(new_image_id, &image).await.unwrap();
    } else {
        panic!("missing field");
    }
    "OK".to_string()
}

async fn insert_image_into_database(
    pool: &sqlx::Pool<sqlx::Sqlite>,
    tags: &str,
) -> anyhow::Result<i64> {
    let row = sqlx::query("INSERT INTO images(tags) values (?) RETURNING id")
        .bind(tags)
        .fetch_one(pool)
        .await?;
    Ok(row.get(0))
}

async fn save_image(id: i64, bytes: &[u8]) -> anyhow::Result<()> {
    let base_path = std::path::Path::new("image");
    if !base_path.exists() || !base_path.is_dir() {
        tokio::fs::create_dir_all(base_path).await?;
    }

    let image_path = base_path.join(format!("{id}.png"));
    if image_path.exists() {
        anyhow::bail!("File already exists");
    }

    tokio::fs::write(image_path, bytes).await?;

    Ok(())
}

async fn get_image(Path(id): axum::extract::Path<i64>) -> impl IntoResponse {
    let filename = format!("image/{id}.jpg");
    let attachment = format!("filename={filename}");
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("image/jpeg"),
    );

    let file = tokio::fs::File::open(filename).await.unwrap();

    axum::response::Response::builder()
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("image/jpeg"),
        )
        .header(
            header::CONTENT_DISPOSITION,
            header::HeaderValue::from_str(&attachment).unwrap(),
        )
        .body(StreamBody::new(ReaderStream::new(file)))
}
