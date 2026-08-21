use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, SimpleObject)]
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
