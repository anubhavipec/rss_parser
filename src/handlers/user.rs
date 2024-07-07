use std::thread::sleep;
use std::time::Duration;
use axum::{Json, extract::Path, http::StatusCode, response::{IntoResponse, Response}, body::Body, Extension};
use axum::body::HttpBody;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::mysql::MySqlPool;
use sqlx::Row;

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
#[derive(Deserialize)]
struct Product{
    id: u32,
    name: String,
    price: f32,
    description: String,
}

pub async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("user created successfully"))
        .unwrap()
}

pub async fn task1() {
    let mut i =0;
    loop {
        if(i<=1000){
            println!("printing i: {i} \n");
            tokio::time::sleep(Duration::from_millis(2)).await;
        }
        else { break; }
        i+=1;
    }
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

    // let mut i = 0;
    //
    // let handle = tokio::spawn( async move {
    //     loop {
    //         if(i<=1000){
    //             println!("printing i: {i} \n");
    //             tokio::time::sleep(Duration::from_millis(2)).await;
    //         }
    //         else { break; }
    //         i+=1;
    //     }
    // });

    let user: Vec<serde_json::Value>  = rows
        .into_iter()
        .map(|row| {
            let id: u32 = row.try_get("id").unwrap_or_default();
            let name: String = row.try_get("name").unwrap_or_default();
            let price: f32 = row.try_get("price").unwrap_or_default();
            let description: String = row.try_get("description").unwrap_or_default();

            json!({
                "id": id,
                "name": name,
                "price": price,
                "description": description,
            })
        })
        .collect();
    println!("Collection data from db");
    // handle.await.unwrap();
    (axum::http::StatusCode::OK,Json(user)).into_response()
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
