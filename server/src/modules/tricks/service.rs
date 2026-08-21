use sqlx::PgPool;
use uuid::Uuid;

use super::dto::CreateTrickInput;
use super::model::TrickModel;
use super::repository::TrickRepository;
use crate::core::errors::AppError;
use crate::modules::spots::service::SpotService;

pub struct TrickService;

impl TrickService {
    pub async fn create_trick(
        pool: &PgPool,
        user_id: Uuid,
        input: CreateTrickInput,
    ) -> Result<TrickModel, AppError> {
        // 1. Délégation de la logique de spot au service dédié
        let spot = SpotService::find_or_create_spot(pool, input.latitude, input.longitude).await?;

        // 2. Persistance via le repository
        TrickRepository::create(pool, user_id, spot.id, input.description, input.video_url).await
    }

    pub async fn approve_trick(pool: &PgPool, trick_id: Uuid) -> Result<bool, AppError> {
        TrickRepository::update_approval(pool, trick_id, true).await
    }

    pub async fn reject_trick(pool: &PgPool, trick_id: Uuid) -> Result<bool, AppError> {
        TrickRepository::delete(pool, trick_id).await
    }

    pub async fn get_approved_tricks_by_spot(
        pool: &PgPool,
        spot_id: Uuid,
    ) -> Result<Vec<TrickModel>, AppError> {
        TrickRepository::find_approved_by_spot(pool, spot_id).await
    }

    pub async fn get_all_approved_tricks(pool: &PgPool) -> Result<Vec<TrickModel>, AppError> {
        TrickRepository::find_all_approved(pool).await
    }

    pub async fn get_pending_tricks(pool: &PgPool) -> Result<Vec<TrickModel>, AppError> {
        TrickRepository::find_pending(pool).await
    }
}
