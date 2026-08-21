use sqlx::PgPool;

use super::dto::{AuthResponseDto, LoginDto, RegisterDto};
use crate::core::errors::AppError;
use crate::core::security::{create_jwt, hash_password, verify_password};
use crate::modules::users::service::UserService;

pub struct AuthService;

impl AuthService {
    pub async fn register(
        pool: &PgPool,
        jwt_secret: &str,
        dto: RegisterDto,
    ) -> Result<AuthResponseDto, AppError> {
        let password_hash = hash_password(&dto.password).map_err(AppError::InternalServerError)?;

        let user = UserService::create_user(
            pool,
            dto.email.to_lowercase().trim().to_string(),
            dto.username.trim().to_string(),
            password_hash,
        )
        .await?;

        let token = create_jwt(&user.id, jwt_secret)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(AuthResponseDto {
            token,
            user_id: user.id,
            username: user.username,
            email: user.email,
            is_admin: user.is_admin,
            avatar_url: user.avatar_url,
        })
    }

    pub async fn login(
        pool: &PgPool,
        jwt_secret: &str,
        dto: LoginDto,
    ) -> Result<AuthResponseDto, AppError> {
        let user = UserService::get_user_by_email(pool, dto.email.to_lowercase().trim())
            .await?
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
            is_admin: user.is_admin,
            avatar_url: user.avatar_url,
        })
    }
}
