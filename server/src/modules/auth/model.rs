use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserModel {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub is_admin: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
