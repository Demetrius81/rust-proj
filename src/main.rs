use axum::{response::Html, routing::get, Router};
use dotenv::dotenv;
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    const VAR_ADDRESS_KEY: &str = "SERVER_ADDRESS";
    dotenv().ok();
    
    let app = Router::new().route("/", get(root));
    let addr = env::var(&VAR_ADDRESS_KEY)
        .unwrap_or_else(|e| panic!("Failed to get env with name '{}': {}", VAR_ADDRESS_KEY, e));
    
    let parset_address: SocketAddr = addr
        .parse()
        .unwrap_or_else(|e| panic!("Failed to parse socket address: {}", e));

    axum_server::bind(parset_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<&'static str> {
    Html("Hello, world!")
}
