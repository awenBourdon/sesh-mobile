use sqlx::PgPool;

use super::dto::CreateSpotInput;
use super::model::SpotModel;
use crate::core::errors::AppError;

pub struct SpotService;

impl SpotService {
    pub async fn create_spot(pool: &PgPool, input: CreateSpotInput) -> Result<SpotModel, AppError> {
        let spot = sqlx::query_as::<_, SpotModel>(
            r#"
            INSERT INTO spots (name, latitude, longitude)
            VALUES ($1, $2, $3)
            RETURNING id, name, latitude, longitude, created_at, updated_at
            "#,
        )
        .bind(input.name)
        .bind(input.latitude)
        .bind(input.longitude)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            AppError::InternalServerError(format!("Error during the creation of the spot : {}", e))
        })?;

        Ok(spot)
    }

    pub async fn get_all_spots(pool: &PgPool) -> Result<Vec<SpotModel>, AppError> {
        let spots = sqlx::query_as::<_, SpotModel>(
            "SELECT id, name, latitude, longitude, created_at, updated_at FROM spots",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(spots)
    }
}
