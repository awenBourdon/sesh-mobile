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

    pub async fn find_or_create_spot(
        pool: &PgPool,
        latitude: f64,
        longitude: f64,
    ) -> Result<SpotModel, AppError> {
        let existing_spot = sqlx::query_as::<_, SpotModel>(
            r#"
            SELECT id, name, latitude, longitude, created_at, updated_at
            FROM spots
            WHERE (
                6371000 * acos(
                    cos(radians($1)) * cos(latitude * PI() / 180) *
                    cos((longitude * PI() / 180) - radians($2)) +
                    sin(radians($1)) * sin(latitude * PI() / 180)
                )
            ) <= 10
            LIMIT 1
            "#,
        )
        .bind(latitude)
        .bind(longitude)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if let Some(spot) = existing_spot {
            Ok(spot)
        } else {
            self::SpotService::create_spot(
                pool,
                CreateSpotInput {
                    name: None,
                    latitude,
                    longitude,
                },
            )
            .await
        }
    }
}
