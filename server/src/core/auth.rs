use crate::core::security::decode_jwt;
use axum::http::HeaderMap;
use axum_extra::extract::cookie::CookieJar;
use uuid::Uuid;

pub struct AuthUser {
    pub id: Uuid,
}

pub fn extract_auth_user(headers: &HeaderMap, jar: &CookieJar, jwt_secret: &str) -> Option<AuthUser> {

    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            println!("   > Found Authorization Header");
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                if let Ok(user_id) = decode_jwt(token, jwt_secret) {
                    return Some(AuthUser { id: user_id });
                }
            }
        }
    }

    if let Some(cookie) = jar.get("admin_token") {
        println!("   > Found admin_token Cookie");
        let token = cookie.value();
        if let Ok(user_id) = decode_jwt(token, jwt_secret) {
            return Some(AuthUser { id: user_id });
        }
        println!("Error: Invalid JWT in Cookie");
    } else {
        println!("> No admin_token Cookie found");
    }
    None
}
