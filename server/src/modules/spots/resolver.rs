use super::dto::CreateSpotInput;
use super::model::SpotModel;
use super::service::SpotService;
use crate::AppState;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Default)]
pub struct SpotsQuery;

#[Object]
impl SpotsQuery {
    async fn get_spots(&self, ctx: &Context<'_>) -> Result<Vec<SpotModel>> {
        let state = ctx.data::<Arc<AppState>>()?;
        let spots = SpotService::get_all_spots(&state.pool)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(spots)
    }

    async fn get_spot_by_id(&self, ctx: &Context<'_>, id: Uuid) -> Result<SpotModel> {
        let state = ctx.data::<Arc<AppState>>()?;
        let spot = SpotService::get_spot_by_id(&state.pool, id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?
            .ok_or_else(|| async_graphql::Error::new("Spot not found"))?;
        Ok(spot)
    }
}

#[derive(Default)]
pub struct SpotsMutation;

#[Object]
impl SpotsMutation {
    async fn create_spot(&self, ctx: &Context<'_>, input: CreateSpotInput) -> Result<SpotModel> {
        let state = ctx.data::<Arc<AppState>>()?;
        let spot = SpotService::create_spot(&state.pool, input)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(spot)
    }
}
