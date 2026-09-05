use super::dto::CreateSpotInput;
use super::model::SpotModel;
use super::service::SpotService;
use crate::AppState;
use crate::core::auth::AuthUser;
use crate::modules::social::service::SocialService;
use crate::modules::tricks::model::TrickModel;
use crate::modules::tricks::service::TrickService;
use async_graphql::{ComplexObject, Context, Object, Result};
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

#[ComplexObject]
impl SpotModel {
    async fn likes_count(&self, ctx: &Context<'_>) -> Result<i64> {
        let state = ctx.data::<Arc<AppState>>()?;
        let count = SocialService::get_spot_likes_count(&state.pool, self.id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(count)
    }

    async fn is_liked_by_me(&self, ctx: &Context<'_>) -> bool {
        let state = match ctx.data::<Arc<AppState>>() {
            Ok(s) => s,
            Err(_) => return false,
        };

        let auth_user = match ctx.data::<AuthUser>() {
            Ok(u) => u,
            Err(_) => return false,
        };

        SocialService::is_spot_liked_by_user(&state.pool, auth_user.id, self.id)
            .await
            .unwrap_or(false)
    }

    async fn comments_count(&self, ctx: &Context<'_>) -> Result<i64> {
        let state = ctx.data::<Arc<AppState>>()?;
        let count = SocialService::get_spot_comments(&state.pool, self.id)
            .await
            .map(|comments| comments.len() as i64)
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(count)
    }

    async fn tricks(&self, ctx: &Context<'_>) -> Result<Vec<TrickModel>> {
        let state = ctx.data::<Arc<AppState>>()?;
        let tricks = TrickService::get_approved_tricks_by_spot(&state.pool, self.id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(tricks)
    }
}
