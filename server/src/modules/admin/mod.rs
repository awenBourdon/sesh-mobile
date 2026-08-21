pub mod controller;
pub mod middleware;

use axum::{
    middleware::from_fn_with_state,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::AppState;

pub fn admin_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let protected_routes = Router::new()
        .route("/", get(controller::dashboard_page))
        .route("/trick-action", post(controller::trick_action_handler))
        .route_layer(from_fn_with_state(state, middleware::admin_middleware));

    Router::new()
        .route("/login", get(controller::login_page).post(controller::login_handler))
        .route("/logout", get(controller::logout_handler))
        .merge(protected_routes)
}
