mod config;
mod core;
mod modules;

use crate::core::auth::extract_auth_user;
use crate::core::graphql_utils::graphql_playground_handler;
use async_graphql::{EmptySubscription, MergedObject, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{Router, extract::State, http::HeaderMap, routing::get};
use axum_extra::extract::cookie::CookieJar;
use config::Config;
use dotenvy::dotenv;
use modules::admin::admin_routes;
use modules::auth::auth_routes;
use modules::social::resolver::{SocialMutation, SocialQuery};
use modules::spots::resolver::{SpotsMutation, SpotsQuery};
use modules::tricks::resolver::{TricksMutation, TricksQuery};
use modules::users::resolver::UsersQuery;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[derive(MergedObject, Default)]
pub struct QueryRoot(SpotsQuery, TricksQuery, UsersQuery, SocialQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(SpotsMutation, TricksMutation, SocialMutation);

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct AppState {
    pub pool: PgPool,
    pub config: Config,
    pub schema: AppSchema,
}

async fn graphql_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    jar: CookieJar,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();

    if let Some(auth_user) = extract_auth_user(&headers, &jar, &state.config.jwt_secret) {
        req = req.data(auth_user);
    }

    state.schema.execute(req.data(state.clone())).await.into()
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

    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .finish();

    let app_state = Arc::new(AppState {
        pool,
        config: config.clone(),
        schema,
    });

    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    let app = Router::new()
        .nest("/api/auth", auth_routes())
        .nest("/admin", admin_routes(app_state.clone()))
        .route(
            "/graphql",
            get(graphql_playground_handler).post(graphql_handler),
        )
        .layer(cors)
        .with_state(app_state);

    let bind_addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("Unable to bind the address");

    println!("Server currently running on port http://{}", bind_addr);
    axum::serve(listener, app).await.unwrap();
}
