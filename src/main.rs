mod handlers;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;

use axum::Router;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(routes::api::api_routes())
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
