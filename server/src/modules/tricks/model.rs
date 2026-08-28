use async_graphql::{ComplexObject, Context, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use uuid::Uuid;

use crate::AppState;
use crate::core::auth::AuthUser;
use crate::modules::social::service::SocialService;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct TrickModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub spot_id: Uuid,
    pub description: Option<String>,
    pub video_url: Option<String>,
    pub is_approved: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[ComplexObject]
impl TrickModel {
    async fn likes_count(&self, ctx: &Context<'_>) -> async_graphql::Result<i64> {
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
            Err(_) => return false, // Pas connecté = pas liké
        };

        SocialService::is_trick_liked_by_user(&state.pool, auth_user.id, self.id)
            .await
            .unwrap_or(false)
    }

    async fn comments_count(&self, ctx: &Context<'_>) -> async_graphql::Result<i64> {
        let state = ctx.data::<Arc<AppState>>()?;
        let count = SocialService::get_trick_comments(&state.pool, self.id)
            .await
            .map(|comments| comments.len() as i64)
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(count)
    }
}
