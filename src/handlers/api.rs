use crate::models::payload::Payload;
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use crate::services::item_service::ItemService;

// 서비스 인스턴스 생성
lazy_static::lazy_static! {
    static ref ITEM_SERVICE: ItemService = ItemService::new();
}

pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

pub async fn hello(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello, {}!", name)
}

pub async fn echo(Json(body): Json<Payload>) -> impl IntoResponse {
    (StatusCode::CREATED, body.message)
}

pub async fn create_item(Json(body): Json<Payload>) -> impl IntoResponse {
    match ITEM_SERVICE.create_item(body).await {
        Ok(message) => (StatusCode::CREATED, message),
        Err(e) => (StatusCode::BAD_REQUEST, e),
    }
}

pub async fn get_item(Path(id): Path<i32>) -> impl IntoResponse {
    match ITEM_SERVICE.get_item(id).await {
        Ok(message) => (StatusCode::OK, message),
        Err(e) => (StatusCode::BAD_REQUEST, e),
    }
}

pub async fn update_item(
    Path(id): Path<i32>,
    Json(body): Json<Payload>,
) -> impl IntoResponse {
    match ITEM_SERVICE.update_item(id, body).await {
        Ok(message) => (StatusCode::OK, message),
        Err(e) => (StatusCode::BAD_REQUEST, e),
    }
}

pub async fn delete_item(Path(id): Path<i32>) -> impl IntoResponse {
    match ITEM_SERVICE.delete_item(id).await {
        Ok(message) => (StatusCode::OK, message),
        Err(e) => (StatusCode::BAD_REQUEST, e),
    }
}
