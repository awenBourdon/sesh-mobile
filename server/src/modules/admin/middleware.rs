use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response, Redirect},
};
use axum_extra::extract::cookie::CookieJar;
use std::sync::Arc;
use crate::AppState;
use crate::core::security::decode_jwt;

pub async fn admin_middleware(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    req: Request<Body>,
    next: Next,
) -> Result<Response, Response> {
    let token = match jar.get("admin_token").map(|c| c.value().to_string()) {
        Some(t) => t,
        None => return Ok(Redirect::to("/admin/login").into_response()),
    };

    let user_id = match decode_jwt(&token, &state.config.jwt_secret) {
        Ok(id) => id,
        Err(_) => return Ok(Redirect::to("/admin/login").into_response()),
    };

    let is_admin = sqlx::query_scalar::<_, bool>("SELECT is_admin FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    if is_admin {
        Ok(next.run(req).await)
    } else {
        Ok(Redirect::to("/admin/login").into_response())
    }
}
