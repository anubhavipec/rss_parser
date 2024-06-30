use axum::{Json, Router, ServiceExt};
use axum::body::Body;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "hello Rust" }))
        .route("/create-user",post(create_user))
        .route("/users",get(list_users))
        .route("/item/:id",get(show_item))
        .route("/add-item",post(add_item))
        .route("/delete-user/:user_id", delete(delete_user));

    println!("Running on http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct User{
    id: u64,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct Page{
    number: u32,
}

#[derive(Deserialize)]
struct Item{
    title: String,
}

async fn delete_user(Path(id): Path<u32>) -> Result<Json<User>, impl  IntoResponse> {
    match perform_delete_user(id as u64).await {
        Ok(_) => Ok(Json(User{
            id: id as u64,
            name: "deleted_user".into(),
            email: "".to_string(),
        })),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR,format!("Failed to delete user"))
        )
    }
} 

async fn perform_delete_user(user_id: u64) -> Result<(),String> {
    if user_id == 1 {
        Err("cannot delete admin".to_string())
    } else {
        Ok(())
    }
}

async fn add_item(Json(item):Json<Item>)-> String{
    format!("Added item {}",item.title)
}

async fn show_item(Path(id):Path<u32>,Query(page): Query<Page>) -> String {
    format!("Items {} on page {}",id,page.number)
}


async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("user created successfully"))
        .unwrap()
}

async fn list_users() -> Json<Vec<User>> {
    let users = vec![User{
        id: 1,
        name: "anubhav".to_string(),
        email: "anubhav".to_string()
    },
                     User{
                         id: 2,
                         name: "anshu".to_string(),
                         email: "anubhav".to_string()
                     },];
    Json(users)
}