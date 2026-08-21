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
    println!("--- [AUTH EXTRACTION DEBUG] ---");

    // 1. Check Authorization Header (Mobile)
    if let Some(token) = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
    {
        println!("   > Found Authorization Header");
        if let Ok(user_id) = decode_jwt(token, jwt_secret) {
            println!("   ✅ Success: User {} identified via Header", user_id);
            return Some(AuthUser { id: user_id });
        }
        println!("   ❌ Error: Invalid JWT in Header");
    }

    // 2. Check Admin Cookie (Dashboard)
    if let Some(cookie) = jar.get("admin_token") {
        println!("   > Found admin_token Cookie");
        let token = cookie.value();
        if let Ok(user_id) = decode_jwt(token, jwt_secret) {
            println!("   ✅ Success: User {} identified via Cookie", user_id);
            return Some(AuthUser { id: user_id });
        }
        println!("   ❌ Error: Invalid JWT in Cookie");
    } else {
        println!("   > No admin_token Cookie found");
    }

    println!("   ⚠️ Final Result: No user identified");
    println!("---------------------------------");
    None
}
