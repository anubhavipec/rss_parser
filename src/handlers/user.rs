use axum::{Json, extract::Path, http::StatusCode, response::{IntoResponse, Response}, body::Body, Extension};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Serialize)]
pub struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct Page {
    number: u32,
}

pub async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("user created successfully"))
        .unwrap()
}

pub async fn list_users(Extension(pool): Extension<MySqlPool>) -> impl IntoResponse {
    let rows = match sqlx::query("SELECT * FROM crud_demo.products")
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => rows,
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error"
                ).into_response()
        }
    };

    let user: Vec<serde_json::Value>  = rows
        .into_iter()
}

pub async fn delete_user(Path(id): Path<u32>) -> Result<Json<User>, impl IntoResponse> {
    match perform_delete_user(id as u64).await {
        Ok(_) => Ok(Json(User {
            id: id as u64,
            name: "deleted_user".into(),
            email: "".to_string(),
        })),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete user"),
        )),
    }
}

async fn perform_delete_user(user_id: u64) -> Result<(), String> {
    if user_id == 1 {
        Err("cannot delete admin".to_string())
    } else {
        Ok(())
    }
}
