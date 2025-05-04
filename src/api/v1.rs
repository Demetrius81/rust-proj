use super::handlers;
use axum::{Router, routing::get};

pub fn configure() -> Router {
		Router::new().route("/hello", get(handlers::hello::hello))
}