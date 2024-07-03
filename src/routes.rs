use axum::Router;
use axum::routing::{delete, get, post};


pub fn create_router() -> Router {
    Router::new()
        .route("/", get(|| async { "hello Rust" }))
        .route("/create-user", post(crate::handlers::user::create_user))
        .route("/users", get(crate::handlers::user::list_users))
        .route("/item/:id", get(crate::handlers::item::show_item))
        .route("/add-item", post(crate::handlers::item::add_item))
        .route("/delete-user/:user_id", delete(crate::handlers::user::delete_user))
}
