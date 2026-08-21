use super::model::UserModel;
use crate::core::errors::AppError;
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
    pub async fn create(
        pool: &PgPool,
        email: String,
        username: String,
        password_hash: String,
    ) -> Result<UserModel, AppError> {
        sqlx::query_as::<_, UserModel>(
            r#"
            INSERT INTO users (email, username, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, email, username, password_hash, is_admin, avatar_url, created_at, updated_at
            "#,
        )
        .bind(email)
        .bind(username)
        .bind(password_hash)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::BadRequest(format!("Database error during user registration: {}", e)))
    }

    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<UserModel>, AppError> {
        sqlx::query_as::<_, UserModel>(
            "SELECT id, email, username, password_hash, is_admin, avatar_url, created_at, updated_at FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<UserModel>, AppError> {
        sqlx::query_as::<_, UserModel>(
            "SELECT id, email, username, password_hash, is_admin, avatar_url, created_at, updated_at FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
    }
}
