mod config;
mod core;
mod modules;

use axum::Router;
use config::Config;
use dotenvy::dotenv;
use modules::auth::auth_routes;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

pub struct AppState {
    pub pool: PgPool,
    pub config: Config,
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

    // Lancer automatiquement les migrations au démarrage du serveur
    // TODO : à supprimer une fois backend bien abouti
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("SQL migrations fails");

    println!("The connection is successful");

    let app_state = Arc::new(AppState {
        pool,
        config: config.clone(),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/api/auth", auth_routes())
        .layer(cors)
        .with_state(app_state);

    let bind_addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("Unabnle to bind the adress");

    println!("Server currently running on port http://{}", bind_addr);
    axum::serve(listener, app).await.unwrap();
}
