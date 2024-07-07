use axum::{Router, Server};
use std::net::SocketAddr;
use sqlx::MySqlPool;
use tracing::instrument::WithSubscriber;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod routes;
mod handlers;

#[tokio::main]
async fn main() {


    let app = routes::create_router().await;

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Running on http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
