use axum::{Router, Server};
use std::net::SocketAddr;
use sqlx::MySqlPool;

mod routes;
mod handlers;

#[tokio::main]
async fn main() {

    let database_url = "mysql://root:Qwerty@123@localhost:3306/test";
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Cannot connect to db");
    let app = routes::create_router();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Running on http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
