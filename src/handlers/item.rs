use axum::{Json, extract::{Path, Query}};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Item {
    title: String,
}

#[derive(Deserialize)]
pub struct Page {
    number: u32,
}

pub async fn add_item(Json(item): Json<Item>) -> String {
    format!("Added item {}", item.title)
}

pub async fn show_item(Path(id): Path<u32>, Query(page): Query<Page>) -> String {
    format!("Items {} on page {}", id, page.number)
}
