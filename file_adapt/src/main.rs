use axum::{
    body::Body,
    http::{HeaderMap, header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use tokio::io::{AsyncBufReadExt, BufReader};
use std::net::SocketAddr;
use axum::http::HeaderValue;
use tokio_stream::StreamExt;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    let file = match tokio::fs::File::open("Cargo.toml").await {
        Ok(file) => file,
        Err(e) => return (StatusCode::NOT_FOUND, format!("File not found: {}", e)).into_response(),
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();
    let stream = tokio_stream::wrappers::LinesStream::new(lines)
        .map(|result| result.map(|line| line.to_uppercase() + "\n"));

    let body = Body::from_stream(stream);
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/plain; charset=utf-8"));
    if let Ok(content_disposition) = HeaderValue::from_str("attachment; filename=\"file.txt\"") {
        headers.insert(header::CONTENT_DISPOSITION, content_disposition);
    }

    (headers, body).into_response()
}