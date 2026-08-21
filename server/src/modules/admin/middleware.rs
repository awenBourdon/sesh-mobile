use crate::AppState;
use crate::core::security::decode_jwt;
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::cookie::CookieJar;
use std::sync::Arc;

pub async fn admin_middleware(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    req: Request<Body>,
    next: Next,
) -> Response {
    let token = match jar.get("admin_token").map(|c| c.value().to_string()) {
        Some(t) => t,
        None => return Redirect::to("/admin/login").into_response(),
    };

    let user_id = match decode_jwt(&token, &state.config.jwt_secret) {
        Ok(id) => id,
        Err(_) => return Redirect::to("/admin/login").into_response(),
    };

    let is_admin_result = sqlx::query_scalar::<_, bool>("SELECT is_admin FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&state.pool)
        .await;

    match is_admin_result {
        Ok(true) => next.run(req).await,
        Ok(false) => Redirect::to("/admin/login").into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
