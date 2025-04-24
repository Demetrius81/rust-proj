use anyhow::Context;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    message: &'static str,
}

struct AppError(anyhow::Error);

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        Self(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/hello", get(hello_json))
        .layer(tower_http::catch_panic::CatchPanicLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.context(">>> Failed to bind TCP listener")?;

    axum::serve(listener, app).await.context(">>> axum::serve failed")?;

    Ok(())
}

async fn hello_json() -> Result<(StatusCode, Json<Response>), AppError> {
    let response = Response {
        message: genetate_message().context(">>> Failed to generate message")?,
    };

    Ok((StatusCode::OK, Json(response)))
}

fn genetate_message() -> anyhow::Result<&'static str> {
    if rand::random() {
        anyhow::bail!("no message")
    }

    Ok("Hello <strong>World!!!</strong>")
}
