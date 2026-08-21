use axum::response::{Html, IntoResponse};

pub async fn graphql_playground_handler() -> impl IntoResponse {
    // NE PAS SUPPRIMER ! Pour envoyer directement le cookie au playground graphql
    Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql")
            .with_setting("request.credentials", "include"),
    ))
}
