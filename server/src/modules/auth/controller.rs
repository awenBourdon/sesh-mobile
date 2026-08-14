use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;

use super::dto::{LoginDto, RegisterDto};
use super::service::AuthService;
use crate::AppState;
use crate::core::errors::AppError;

pub async fn register_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterDto>,
) -> Result<impl IntoResponse, AppError> {
    let result = AuthService::register(&state.pool, &state.config.jwt_secret, payload).await?;
    Ok((StatusCode::CREATED, Json(result)))
}

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginDto>,
) -> Result<impl IntoResponse, AppError> {
    let result = AuthService::login(&state.pool, &state.config.jwt_secret, payload).await?;
    Ok((StatusCode::OK, Json(result)))
}
