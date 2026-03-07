use crate::AppState;
use crate::core::controllers::health::health;
use crate::core::controllers::login_user::login_user;
use crate::core::controllers::logout_user::logout_user;

use crate::core::controllers::register_user::register_user;
use axum::{Router, routing::get, routing::post};
use tower_cookies::CookieManagerLayer;

pub fn auth_routes(_state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route("/logout", post(logout_user))
        .layer(CookieManagerLayer::new())
}
