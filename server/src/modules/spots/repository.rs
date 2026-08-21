use super::dto::CreateSpotInput;
use super::model::SpotModel;
use crate::core::errors::AppError;
use sqlx::PgPool;
use uuid::Uuid;

pub struct SpotRepository;

impl SpotRepository {
    pub async fn create(pool: &PgPool, input: CreateSpotInput) -> Result<SpotModel, AppError> {
        sqlx::query_as::<_, SpotModel>(
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
            AppError::InternalServerError(format!("Database error during spot creation: {}", e))
        })
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<SpotModel>, AppError> {
        sqlx::query_as::<_, SpotModel>(
            "SELECT id, name, latitude, longitude, created_at, updated_at FROM spots",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<SpotModel>, AppError> {
        sqlx::query_as::<_, SpotModel>(
            "SELECT id, name, latitude, longitude, created_at, updated_at FROM spots WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    pub async fn find_nearby(
        pool: &PgPool,
        latitude: f64,
        longitude: f64,
        radius_meters: f64,
    ) -> Result<Option<SpotModel>, AppError> {
        sqlx::query_as::<_, SpotModel>(
            r#"
            SELECT id, name, latitude, longitude, created_at, updated_at
            FROM spots
            WHERE (
                6371000 * acos(
                    cos(radians($1)) * cos(latitude * PI() / 180) *
                    cos((longitude * PI() / 180) - radians($2)) +
                    sin(radians($1)) * sin(latitude * PI() / 180)
                )
            ) <= $3
            LIMIT 1
            "#,
        )
        .bind(latitude)
        .bind(longitude)
        .bind(radius_meters)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
    }
}
