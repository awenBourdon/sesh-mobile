use super::dto::CreateTrickInput;
use super::model::TrickModel;
use super::service::TrickService;
use crate::AppState;
use crate::core::auth::AuthUser;
use crate::modules::social::service::SocialService;
use crate::modules::spots::model::SpotModel;
use crate::modules::spots::service::SpotService;
use async_graphql::{ComplexObject, Context, Object, Result};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Default)]
pub struct TricksQuery;

#[Object]
impl TricksQuery {
    async fn get_tricks_by_spot(
        &self,
        ctx: &Context<'_>,
        spot_id: Uuid,
    ) -> Result<Vec<TrickModel>> {
        let state = ctx.data::<Arc<AppState>>()?;
        let tricks = TrickService::get_approved_tricks_by_spot(&state.pool, spot_id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(tricks)
    }

    async fn get_all_tricks(&self, ctx: &Context<'_>) -> Result<Vec<TrickModel>> {
        let state = ctx.data::<Arc<AppState>>()?;
        let tricks = TrickService::get_all_approved_tricks(&state.pool)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(tricks)
    }
}

#[derive(Default)]
pub struct TricksMutation;

#[Object]
impl TricksMutation {
    async fn create_trick(&self, ctx: &Context<'_>, input: CreateTrickInput) -> Result<TrickModel> {
        let state = ctx.data::<Arc<AppState>>()?;

        let auth_user = ctx.data::<AuthUser>().map_err(|_| {
            async_graphql::Error::new("Unauthorized: You must be logged in to create a trick")
        })?;

        let trick = TrickService::create_trick(&state.pool, auth_user.id, input)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(trick)
    }

    async fn approve_trick(&self, ctx: &Context<'_>, trick_id: Uuid) -> Result<bool> {
        let state = ctx.data::<Arc<AppState>>()?;
        // TODO: Vérifier si l'utilisateur est ADMIN
        TrickService::approve_trick(&state.pool, trick_id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    async fn reject_trick(&self, ctx: &Context<'_>, trick_id: Uuid) -> Result<bool> {
        let state = ctx.data::<Arc<AppState>>()?;
        // TODO: Vérifier si l'utilisateur est ADMIN
        TrickService::reject_trick(&state.pool, trick_id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }
}

#[ComplexObject]
impl TrickModel {
    async fn likes_count(&self, ctx: &Context<'_>) -> Result<i64> {
        let state = ctx.data::<Arc<AppState>>()?;
        let count = SocialService::get_trick_likes_count(&state.pool, self.id)
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

        SocialService::is_trick_liked_by_user(&state.pool, auth_user.id, self.id)
            .await
            .unwrap_or(false)
    }

    async fn comments_count(&self, ctx: &Context<'_>) -> Result<i64> {
        let state = ctx.data::<Arc<AppState>>()?;
        let count = SocialService::get_trick_comments(&state.pool, self.id)
            .await
            .map(|comments| comments.len() as i64)
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(count)
    }

    async fn spot(&self, ctx: &Context<'_>) -> Result<Option<SpotModel>> {
        let state = ctx.data::<Arc<AppState>>()?;
        let spot = SpotService::get_spot_by_id(&state.pool, self.spot_id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(spot)
    }
}
