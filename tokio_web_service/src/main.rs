use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use serde::Serialize;

#[derive(Serialize, Clone)]
struct HelloJson {
    message: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(say_hello_text))
        .route("/hello", get(say_hello_html))
        .route("/hellof", get(say_hello_html_file))
        .route("/helloj", get(say_hello_json))
        .route("/hellop", post(say_hello_json_post))
        .route("/hellod", get(say_hello_html_disk));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8180")
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn say_hello_text() -> &'static str {
    "Hello, World!"
}

async fn say_hello_html() -> Html<&'static str> {
    Html("<h1>Hello, World!<h1>")
}

async fn say_hello_html_file() -> Html<&'static str> {
    const HTML: &'static str = include_str!("hello.html");
    Html(HTML)
}

async fn say_hello_html_disk() -> Html<String> {
    let path = std::path::Path::new("src/hello.html");
    let contents = tokio::fs::read_to_string(path).await.unwrap();
    Html(contents)
}

async fn say_hello_json() -> axum::Json<HelloJson> {
    let message = HelloJson {
        message: "Hello, Json!".to_string(),
    };
    axum::Json(message)
}
async fn say_hello_json_post() -> axum::Json<HelloJson> {
    let message = HelloJson {
        message: "Hello, post!".to_string(),
    };
    axum::Json(message)
}
