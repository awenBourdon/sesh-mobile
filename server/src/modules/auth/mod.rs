pub mod controller;
pub mod dto;
pub mod model;
pub mod service;

use crate::AppState;
use axum::{Router, routing::post};
use std::sync::Arc;

pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(controller::register_handler))
        .route("/login", post(controller::login_handler))
}
