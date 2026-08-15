mod config;
mod core;
mod modules;

use async_graphql::{EmptySubscription, MergedObject, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};
use config::Config;
use dotenvy::dotenv;
use modules::auth::auth_routes;
use modules::spots::resolver::{SpotsMutation, SpotsQuery};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

#[derive(MergedObject, Default)]
pub struct QueryRoot(SpotsQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(SpotsMutation);

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct AppState {
    pub pool: PgPool,
    pub config: Config,
    pub schema: AppSchema,
}

async fn graphql_handler(
    State(state): State<Arc<AppState>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    state.schema.execute(req.into_inner().data(state.clone())).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
    ))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config::init();

    println!("⏳ Connect to NeonDB...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Impossible to connect to NeonDB");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("SQL migrations fails");

    println!("The connection is successful");

    let schema = Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .finish();

    let app_state = Arc::new(AppState {
        pool,
        config: config.clone(),
        schema,
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/api/auth", auth_routes())
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .layer(cors)
        .with_state(app_state);

    let bind_addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("Unable to bind the address");

    println!("Server currently running on port http://{}", bind_addr);
    axum::serve(listener, app).await.unwrap();
}
