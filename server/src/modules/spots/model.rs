use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, SimpleObject)]
pub struct SpotModel {
    pub id: Uuid,
    pub name: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
