use axum::{Json, Router, routing::get};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    message: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/auth/health",
    responses(
        (status = 200, description = "Chat Auth Health Endpoint", body = HealthResponse)
    )
)]
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        message: "Hello, Chat Auth Server is up and running !".to_string(),
    })
}
