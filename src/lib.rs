use crate::core::router::auth_routes;
use crate::middlewares::logging_middleware::logging_middleware;
use crate::middlewares::request_timeout_middleware::timeout_middleware;
use crate::utils::load_config::AppConfig;
use axum::{Router, middleware};
use sqlx::PgPool;
use std::sync::Arc;

pub mod core;
pub mod db;
pub mod middlewares;
pub mod utils;

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db: PgPool,
}

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1/auth", auth_routes(&state))
        .layer(middleware::from_fn(logging_middleware))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            timeout_middleware,
        ))
        .with_state(state)
}
