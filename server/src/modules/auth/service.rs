use sqlx::PgPool;

use super::dto::{AuthResponseDto, LoginDto, RegisterDto};
use super::model::UserModel;
use crate::core::errors::AppError;
use crate::core::security::{create_jwt, hash_password, verify_password};

pub struct AuthService;

impl AuthService {
    pub async fn register(
        pool: &PgPool,
        jwt_secret: &str,
        dto: RegisterDto,
    ) -> Result<AuthResponseDto, AppError> {
        let password_hash = hash_password(&dto.password).map_err(AppError::InternalServerError)?;

        let user = sqlx::query_as::<_, UserModel>(
            r#"
            INSERT INTO users (email, username, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, email, username, password_hash, created_at, updated_at
            "#,
        )
        .bind(dto.email.to_lowercase().trim())
        .bind(dto.username.trim())
        .bind(password_hash)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::BadRequest(format!("Error creating user : {}", e)))?;

        let token = create_jwt(&user.id, jwt_secret)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(AuthResponseDto {
            token,
            user_id: user.id,
            username: user.username,
            email: user.email,
        })
    }

    pub async fn login(
        pool: &PgPool,
        jwt_secret: &str,
        dto: LoginDto,
    ) -> Result<AuthResponseDto, AppError> {
        let user = sqlx::query_as::<_, UserModel>(
            "SELECT id, email, username, password_hash, created_at, updated_at FROM users WHERE email = $1",
        )
        .bind(dto.email.to_lowercase().trim())
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?
        .ok_or_else(|| AppError::Unauthorized("Incorrect email address or password".into()))?;

        if !verify_password(&dto.password, &user.password_hash) {
            return Err(AppError::Unauthorized(
                "Incorrect email address or password".into(),
            ));
        }

        let token = create_jwt(&user.id, jwt_secret)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(AuthResponseDto {
            token,
            user_id: user.id,
            username: user.username,
            email: user.email,
        })
    }
}
