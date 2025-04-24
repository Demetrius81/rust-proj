use std::net::SocketAddr;
use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

// #[tokio::main]
// async fn main() {
//     let routes_hello = Router::new().route("/hello", get(|| async {Html("Hello <strong>World!!!</strong>")}));

//     // region:   --- Start server

//     let addr = SocketAddr::from(([127,0,0,1], 8080));
//     println!("--->>>LISTENING on {addr}\n");
//     axum_server::bind(addr)
//         .serve(routes_hello.into_make_service())
//         .await
//         .unwrap();

//     // endregion:    --- Start server
// }

async fn hello() -> &'static str {
    "Hello <strong>World!!!</strong>"
}
