use async_graphql::InputObject;
use serde::Deserialize;

#[derive(Debug, Deserialize, InputObject)]
pub struct CreateSpotInput {
    pub name: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
}
