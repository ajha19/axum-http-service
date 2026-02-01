use axum::{routing::get, Router};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;

pub mod handlers;
pub mod middleware;
pub mod metrics_state;

use handlers::health::health_check;
use middleware::{
    metrics::MetricsLayer,
    rate_limit::RateLimitLayer,
    request_id::RequestIdLayer,
};
use metrics_state::MetricsState;
use axum::{Json, Extension};

pub fn app() -> Router {
    let metrics_state = MetricsState::new();

    let api_middleware = ServiceBuilder::new()
        .layer(RequestIdLayer)
        .layer(MetricsLayer { state: metrics_state.clone() })
        .layer(RateLimitLayer::new(5, Duration::from_secs(10), metrics_state.clone())); // Burst of 5, refill slowly

    // API Routes (Rate Limited)
    let api_routes = Router::new()
        .route("/health", get(health_check))
        .layer(api_middleware);

    // Static Asset Routes & Metrics (No Rate Limit)
    // We use a State extractor or just capture it. 
    // Since we used ServiceBuilder with Layers, passing state down via Extension is cleaner,
    // but here we are in same scope.
    // Let's use Extension for metrics endpoint for simplicity if simple capture fails.
    // Actually, distinct function is best.

    Router::new()
        .nest_service("/", ServeDir::new("static"))
        .route("/metrics", get(metrics_handler))
        .layer(Extension(metrics_state))
        .merge(api_routes)
}

// #[axum::debug_handler] - feature not enabled, removed.
async fn metrics_handler(Extension(state): Extension<MetricsState>) -> Json<metrics_state::MetricsSnapshot> {
    Json(state.snapshot())
}
