use super::model::UserModel;
use super::service::UserService;
use crate::AppState;
use crate::core::auth::AuthUser;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

#[derive(Default)]
pub struct UsersQuery;

#[Object]
impl UsersQuery {
    async fn me(&self, ctx: &Context<'_>) -> Result<UserModel> {
        let state = ctx.data::<Arc<AppState>>()?;
        let auth_user = ctx
            .data::<AuthUser>()
            .map_err(|_| async_graphql::Error::new("Unauthorized"))?;

        let user = UserService::get_user_by_id(&state.pool, auth_user.id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?
            .ok_or_else(|| async_graphql::Error::new("User not found"))?;

        Ok(user)
    }
}
