use async_graphql::{Context, Object, Result};
use crate::core::auth::AuthUser;
use crate::AppState;
use std::sync::Arc;
use uuid::Uuid;
use super::dto::CreateTrickInput;
use super::model::TrickModel;
use super::service::TrickService;

#[derive(Default)]
pub struct TricksQuery;

#[Object]
impl TricksQuery {
    async fn get_tricks_by_spot(&self, ctx: &Context<'_>, spot_id: Uuid) -> Result<Vec<TrickModel>> {
        let state = ctx.data::<Arc<AppState>>()?;
        let tricks = TrickService::get_approved_tricks_by_spot(&state.pool, spot_id).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(tricks)
    }

    async fn get_all_tricks(&self, ctx: &Context<'_>) -> Result<Vec<TrickModel>> {
        let state = ctx.data::<Arc<AppState>>()?;
        println!("--- [QUERY DEBUG] Fetching all approved tricks ---");
        let tricks = TrickService::get_all_approved_tricks(&state.pool).await
            .map_err(|e| {
                println!("   ❌ Error: {}", e);
                async_graphql::Error::new(e.to_string())
            })?;
        println!("   ✅ Success: {} tricks found", tricks.len());
        Ok(tricks)
    }
}

#[derive(Default)]
pub struct TricksMutation;

#[Object]
impl TricksMutation {
    async fn create_trick(&self, ctx: &Context<'_>, input: CreateTrickInput) -> Result<TrickModel> {
        let state = ctx.data::<Arc<AppState>>()?;

        // On récupère l'utilisateur authentifié depuis le contexte
        let auth_user = ctx.data::<AuthUser>()
            .map_err(|_| async_graphql::Error::new("Unauthorized: You must be logged in to create a trick"))?;

        let trick = TrickService::create_trick(&state.pool, auth_user.id, input).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(trick)
    }

    async fn approve_trick(&self, ctx: &Context<'_>, trick_id: Uuid) -> Result<bool> {
        let state = ctx.data::<Arc<AppState>>()?;
        // TODO: Vérifier si l'utilisateur est ADMIN
        TrickService::approve_trick(&state.pool, trick_id).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    async fn reject_trick(&self, ctx: &Context<'_>, trick_id: Uuid) -> Result<bool> {
        let state = ctx.data::<Arc<AppState>>()?;
        // TODO: Vérifier si l'utilisateur est ADMIN
        TrickService::reject_trick(&state.pool, trick_id).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }
}
