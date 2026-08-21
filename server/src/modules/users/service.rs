use super::model::UserModel;
use super::repository::UserRepository;
use crate::core::errors::AppError;
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserService;

impl UserService {
    pub async fn create_user(
        pool: &PgPool,
        email: String,
        username: String,
        password_hash: String,
    ) -> Result<UserModel, AppError> {
        UserRepository::create(pool, email, username, password_hash).await
    }

    pub async fn get_user_by_email(
        pool: &PgPool,
        email: &str,
    ) -> Result<Option<UserModel>, AppError> {
        UserRepository::find_by_email(pool, email).await
    }

    pub async fn get_user_by_id(pool: &PgPool, id: Uuid) -> Result<Option<UserModel>, AppError> {
        UserRepository::find_by_id(pool, id).await
    }
}
