use super::model::TrickModel;
use crate::core::errors::AppError;
use sqlx::PgPool;
use uuid::Uuid;

pub struct TrickRepository;

impl TrickRepository {
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        spot_id: Uuid,
        description: Option<String>,
        video_url: Option<String>,
    ) -> Result<TrickModel, AppError> {
        sqlx::query_as::<_, TrickModel>(
            r#"
            INSERT INTO tricks (user_id, spot_id, description, video_url)
            VALUES ($1, $2, $3, $4)
            RETURNING id, user_id, spot_id, description, video_url, is_approved, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(spot_id)
        .bind(description)
        .bind(video_url)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Database error during trick creation: {}", e)))
    }

    pub async fn update_approval(
        pool: &PgPool,
        id: Uuid,
        is_approved: bool,
    ) -> Result<bool, AppError> {
        let result = sqlx::query("UPDATE tricks SET is_approved = $1 WHERE id = $2")
            .bind(is_approved)
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM tricks WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn find_approved_by_spot(
        pool: &PgPool,
        spot_id: Uuid,
    ) -> Result<Vec<TrickModel>, AppError> {
        sqlx::query_as::<_, TrickModel>(
            "SELECT * FROM tricks WHERE spot_id = $1 AND is_approved = true ORDER BY created_at DESC",
        )
        .bind(spot_id)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    pub async fn find_all_approved(pool: &PgPool) -> Result<Vec<TrickModel>, AppError> {
        sqlx::query_as::<_, TrickModel>(
            "SELECT * FROM tricks WHERE is_approved = true ORDER BY created_at DESC",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    pub async fn find_pending(pool: &PgPool) -> Result<Vec<TrickModel>, AppError> {
        sqlx::query_as::<_, TrickModel>(
            "SELECT * FROM tricks WHERE is_approved = false ORDER BY created_at ASC",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
    }
}
