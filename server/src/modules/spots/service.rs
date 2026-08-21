use sqlx::PgPool;

use super::dto::CreateSpotInput;
use super::model::SpotModel;
use super::repository::SpotRepository;
use crate::core::errors::AppError;

pub struct SpotService;

impl SpotService {
    pub async fn create_spot(pool: &PgPool, input: CreateSpotInput) -> Result<SpotModel, AppError> {
        SpotRepository::create(pool, input).await
    }

    pub async fn get_all_spots(pool: &PgPool) -> Result<Vec<SpotModel>, AppError> {
        SpotRepository::find_all(pool).await
    }

    pub async fn find_or_create_spot(
        pool: &PgPool,
        latitude: f64,
        longitude: f64,
    ) -> Result<SpotModel, AppError> {
        let radius = 10.0;

        let existing_spot = SpotRepository::find_nearby(pool, latitude, longitude, radius).await?;

        if let Some(spot) = existing_spot {
            Ok(spot)
        } else {
            SpotRepository::create(
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
