use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
}

pub async fn health_check() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "healthy".to_string(),
        }),
    )
}
