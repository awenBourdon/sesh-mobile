use super::model::CommentModel;
use super::service::SocialService;
use crate::AppState;
use crate::core::auth::AuthUser;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Default)]
pub struct SocialQuery;

#[Object]
impl SocialQuery {
    async fn get_trick_comments(
        &self,
        ctx: &Context<'_>,
        trick_id: Uuid,
    ) -> Result<Vec<CommentModel>> {
        let state = ctx.data::<Arc<AppState>>()?;
        let comments = SocialService::get_trick_comments(&state.pool, trick_id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(comments)
    }

    async fn get_trick_likes_count(&self, ctx: &Context<'_>, trick_id: Uuid) -> Result<i64> {
        let state = ctx.data::<Arc<AppState>>()?;
        SocialService::get_trick_likes_count(&state.pool, trick_id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    async fn get_spot_comments(
        &self,
        ctx: &Context<'_>,
        spot_id: Uuid,
    ) -> Result<Vec<CommentModel>> {
        let state = ctx.data::<Arc<AppState>>()?;
        let comments = SocialService::get_spot_comments(&state.pool, spot_id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(comments)
    }

    async fn get_spot_likes_count(&self, ctx: &Context<'_>, spot_id: Uuid) -> Result<i64> {
        let state = ctx.data::<Arc<AppState>>()?;
        SocialService::get_spot_likes_count(&state.pool, spot_id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }
}

#[derive(Default)]
pub struct SocialMutation;

#[Object]
impl SocialMutation {
    async fn toggle_like(&self, ctx: &Context<'_>, trick_id: Uuid) -> Result<bool> {
        let state = ctx.data::<Arc<AppState>>()?;
        let auth_user = ctx
            .data::<AuthUser>()
            .map_err(|_| async_graphql::Error::new("Unauthorized"))?;

        SocialService::toggle_like(&state.pool, auth_user.id, trick_id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    async fn toggle_spot_like(&self, ctx: &Context<'_>, spot_id: Uuid) -> Result<bool> {
        let state = ctx.data::<Arc<AppState>>()?;
        let auth_user = ctx
            .data::<AuthUser>()
            .map_err(|_| async_graphql::Error::new("Unauthorized"))?;

        SocialService::toggle_spot_like(&state.pool, auth_user.id, spot_id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    async fn add_comment(
        &self,
        ctx: &Context<'_>,
        trick_id: Uuid,
        content: String,
    ) -> Result<CommentModel> {
        let state = ctx.data::<Arc<AppState>>()?;
        let auth_user = ctx
            .data::<AuthUser>()
            .map_err(|_| async_graphql::Error::new("Unauthorized"))?;

        SocialService::add_comment(&state.pool, auth_user.id, trick_id, content)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    async fn add_spot_comment(
        &self,
        ctx: &Context<'_>,
        spot_id: Uuid,
        content: String,
    ) -> Result<CommentModel> {
        let state = ctx.data::<Arc<AppState>>()?;
        let auth_user = ctx
            .data::<AuthUser>()
            .map_err(|_| async_graphql::Error::new("Unauthorized"))?;

        SocialService::add_spot_comment(&state.pool, auth_user.id, spot_id, content)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    async fn delete_comment(&self, ctx: &Context<'_>, comment_id: Uuid) -> Result<bool> {
        let state = ctx.data::<Arc<AppState>>()?;
        let auth_user = ctx
            .data::<AuthUser>()
            .map_err(|_| async_graphql::Error::new("Unauthorized"))?;

        SocialService::delete_comment(&state.pool, auth_user.id, comment_id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    async fn delete_spot_comment(&self, ctx: &Context<'_>, comment_id: Uuid) -> Result<bool> {
        let state = ctx.data::<Arc<AppState>>()?;
        let auth_user = ctx
            .data::<AuthUser>()
            .map_err(|_| async_graphql::Error::new("Unauthorized"))?;

        SocialService::delete_spot_comment(&state.pool, auth_user.id, comment_id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }
}
