use axum::response::Html;
use axum::Json;
use axum::{routing::get, Router};
use serde::Serialize;

async fn hello_html() -> Html<&'static str> {
    Html("<h1>Hello world</h1>")
}

#[derive(Serialize)]
struct HelloJson {
    message: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(hello_html))
        .route("/json", get(hello_json));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn hello_json() -> axum::Json<HelloJson> {
    let message = HelloJson {
        message: "Hello".to_string(),
    };
    Json(message)
}
