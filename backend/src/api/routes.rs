use axum::Router;

use super::handlers;

/// Build the API router with all endpoint routes.
pub fn build_router() -> Router {
    Router::new()
        .route("/api/compress", axum::routing::post(handlers::compress))
        .route("/api/compress/stream", axum::routing::post(handlers::compress_stream))
        .route("/api/decompress", axum::routing::post(handlers::decompress))
        .route("/api/algorithms", axum::routing::get(handlers::list_algorithms))
        .route("/api/health", axum::routing::get(handlers::health_check))
}
