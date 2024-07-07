use axum::{Extension, Router};
use axum::routing::{delete, get, post};
use sqlx::MySqlPool;


pub async fn create_router() -> Router {
    let database_url = "mysql://root:Qwerty@123@localhost:3306/test";
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Cannot connect to db");
    Router::new()
        .route("/", get(|| async { "hello Rust" }))
        .route("/create-user", post(crate::handlers::user::create_user))
        .route("/users", get(crate::handlers::user::list_users))
        .route("/item/:id", get(crate::handlers::item::show_item))
        .route("/add-item", post(crate::handlers::item::add_item))
        .route("/delete-user/:user_id", delete(crate::handlers::user::delete_user))
        .layer(Extension(pool))
}
