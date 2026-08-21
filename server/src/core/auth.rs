use crate::core::security::decode_jwt;
use axum::http::HeaderMap;
use axum_extra::extract::cookie::CookieJar;
use uuid::Uuid;

pub struct AuthUser {
    pub id: Uuid,
}

pub fn extract_auth_user(
    headers: &HeaderMap,
    jar: &CookieJar,
    jwt_secret: &str,
) -> Option<AuthUser> {
    // 1. Check Authorization Header (Mobile)
    if let Some(token) = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
    {
        if let Ok(user_id) = decode_jwt(token, jwt_secret) {
            return Some(AuthUser { id: user_id });
        }
    }

    // 2. Check Admin Cookie (Dashboard)
    if let Some(cookie) = jar.get("admin_token") {
        let token = cookie.value();
        if let Ok(user_id) = decode_jwt(token, jwt_secret) {
            return Some(AuthUser { id: user_id });
        }
    }

    None
}
