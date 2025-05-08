use crate::handlers::api;
use axum::{
    routing::{get, post, put, patch, delete},
    Router,
};

pub fn api_routes() -> Router {
    Router::new()
        .route("/health", get(api::health))
        .route("/hello/:name", get(api::hello))
        .route("/echo", post(api::echo))
        .route("/items", post(api::create_item))
        .route("/items/:id", get(api::get_item))
        .route("/items/:id", put(api::update_item))
        .route("/items/:id", patch(api::update_item))
        .route("/items/:id", delete(api::delete_item))
}
