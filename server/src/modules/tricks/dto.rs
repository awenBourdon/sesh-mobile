use async_graphql::InputObject;
use serde::Deserialize;

#[derive(Debug, Deserialize, InputObject)]
pub struct CreateTrickInput {
    pub latitude: f64,
    pub longitude: f64,
    pub description: Option<String>,
    pub video_url: Option<String>,
}
