use sqlx::PgPool;
use uuid::Uuid;

use super::dto::CreateTrickInput;
use super::model::TrickModel;
use crate::core::errors::AppError;
use crate::modules::spots::service::SpotService;

pub struct TrickService;

impl TrickService {
    pub async fn create_trick(
        pool: &PgPool,
        user_id: Uuid,
        input: CreateTrickInput,
    ) -> Result<TrickModel, AppError> {
        let spot = SpotService::find_or_create_spot(pool, input.latitude, input.longitude).await?;

        let trick = sqlx::query_as::<_, TrickModel>(
            r#"
            INSERT INTO tricks (user_id, spot_id, description, video_url)
            VALUES ($1, $2, $3, $4)
            RETURNING id, user_id, spot_id, description, video_url, is_approved, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(spot.id)
        .bind(input.description)
        .bind(input.video_url)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Error during trick creation : {}", e)))?;

        Ok(trick)
    }

    pub async fn approve_trick(pool: &PgPool, trick_id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query("UPDATE tricks SET is_approved = true WHERE id = $1")
            .bind(trick_id)
            .execute(pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn reject_trick(pool: &PgPool, trick_id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM tricks WHERE id = $1")
            .bind(trick_id)
            .execute(pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_approved_tricks_by_spot(
        pool: &PgPool,
        spot_id: Uuid,
    ) -> Result<Vec<TrickModel>, AppError> {
        let tricks = sqlx::query_as::<_, TrickModel>(
            "SELECT * FROM tricks WHERE spot_id = $1 AND is_approved = true ORDER BY created_at DESC",
        )
        .bind(spot_id)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(tricks)
    }

    pub async fn get_all_approved_tricks(pool: &PgPool) -> Result<Vec<TrickModel>, AppError> {
        let tricks = sqlx::query_as::<_, TrickModel>(
            "SELECT * FROM tricks WHERE is_approved = true ORDER BY created_at DESC",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(tricks)
    }

    pub async fn get_pending_tricks(pool: &PgPool) -> Result<Vec<TrickModel>, AppError> {
        let tricks = sqlx::query_as::<_, TrickModel>(
            "SELECT * FROM tricks WHERE is_approved = false ORDER BY created_at ASC",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(tricks)
    }
}
