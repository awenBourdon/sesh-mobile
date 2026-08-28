use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, SimpleObject)]
pub struct CommentModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub trick_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LikeModel {
    pub user_id: Uuid,
    pub trick_id: Uuid,
    pub created_at: DateTime<Utc>,
}
