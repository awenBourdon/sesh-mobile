use axum::{
    Form,
    extract::State,
    response::{Html, IntoResponse, Redirect},
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::AppState;
use crate::modules::auth::dto::LoginDto;
use crate::modules::auth::service::AuthService;
use crate::modules::tricks::service::TrickService;

#[derive(Deserialize)]
pub struct AdminLoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct TrickActionForm {
    pub trick_id: Uuid,
    pub action: String,
}

pub async fn login_page(State(state): State<Arc<AppState>>, jar: CookieJar) -> impl IntoResponse {
    // Si déjà connecté et admin, redirection vers le dashboard
    let already_logged_in = jar
        .get("admin_token")
        .map(|cookie| {
            crate::core::security::decode_jwt(cookie.value(), &state.config.jwt_secret).is_ok()
        })
        .unwrap_or(false);

    if already_logged_in {
        return Redirect::to("/admin").into_response();
    }

    Html(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Sesh Admin - Login</title>
            <style>
                body { font-family: sans-serif; display: flex; justify-content: center; align-items: center; height: 100vh; background: #0b0f17; color: white; }
                form { background: #1a1f2e; padding: 2rem; border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.3); width: 300px; }
                div { margin-bottom: 1rem; }
                label { display: block; margin-bottom: 0.5rem; color: #a0aec0; }
                input { width: 100%; padding: 0.5rem; background: #2d3748; border: 1px solid #4a5568; border-radius: 4px; color: white; box-sizing: border-box; }
                button { width: 100%; padding: 0.75rem; background: #4299e1; color: white; border: none; border-radius: 4px; cursor: pointer; font-weight: bold; }
                button:hover { background: #3182ce; }
                h2 { text-align: center; margin-top: 0; color: #fff; }
            </style>
        </head>
        <body>
            <form action="/admin/login" method="POST">
                <h2>Sesh Admin</h2>
                <div>
                    <label>Email</label>
                    <input type="email" name="email" required>
                </div>
                <div>
                    <label>Password</label>
                    <input type="password" name="password" required>
                </div>
                <button type="submit">Se connecter</button>
            </form>
        </body>
        </html>
    "#).into_response()
}

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Form(payload): Form<AdminLoginForm>,
) -> impl IntoResponse {
    let login_dto = LoginDto {
        email: payload.email,
        password: payload.password,
    };

    match AuthService::login(&state.pool, &state.config.jwt_secret, login_dto).await {
        Ok(auth_res) => {
            if auth_res.is_admin {
                let cookie = Cookie::build(("admin_token", auth_res.token))
                    .path("/")
                    .http_only(true)
                    .build();
                (jar.add(cookie), Redirect::to("/admin")).into_response()
            } else {
                Html("<h2>Accès refusé : Vous n'êtes pas administrateur</h2><a href='/admin/login'>Retour</a>").into_response()
            }
        }
        Err(_) => {
            Html("<h2>Identifiants invalides</h2><a href='/admin/login'>Retour</a>").into_response()
        }
    }
}

pub async fn dashboard_page(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let pending_tricks = TrickService::get_pending_tricks(&state.pool)
        .await
        .unwrap_or_default();

    let mut tricks_html = String::new();
    if pending_tricks.is_empty() {
        tricks_html = "<tr><td colspan='4' style='text-align:center; padding: 20px;'>Aucun trick en attente de validation.</td></tr>".to_string();
    } else {
        for trick in pending_tricks {
            tricks_html.push_str(&format!(
                r#"
                <tr>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>
                        <form action="/admin/trick-action" method="POST" style="display:inline;">
                            <input type="hidden" name="trick_id" value="{}">
                            <button type="submit" name="action" value="approve" class="btn-approve">Approuver</button>
                            <button type="submit" name="action" value="reject" class="btn-reject">Rejeter</button>
                        </form>
                    </td>
                </tr>
                "#,
                trick.created_at.unwrap_or_default().format("%d/%m/%Y %H:%M"),
                trick.description.as_deref().unwrap_or("Sans description"),
                trick.spot_id,
                trick.id
            ));
        }
    }

    Html(format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Sesh Admin - Dashboard</title>
            <style>
                body {{ font-family: sans-serif; margin: 0; background: #0b0f17; color: white; }}
                nav {{ background: #1a1f2e; padding: 1rem 2rem; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid #2d3748; }}
                .container {{ padding: 2rem; max-width: 1000px; margin: 0 auto; }}
                h1 {{ color: #fff; }}
                table {{ width: 100%; border-collapse: collapse; background: #1a1f2e; border-radius: 8px; overflow: hidden; margin-top: 20px; }}
                th, td {{ padding: 12px 15px; text-align: left; border-bottom: 1px solid #2d3748; }}
                th {{ background: #2d3748; color: #a0aec0; text-transform: uppercase; font-size: 12px; }}
                .btn-approve {{ background: #48bb78; color: white; border: none; padding: 5px 10px; border-radius: 4px; cursor: pointer; margin-right: 5px; }}
                .btn-reject {{ background: #f56565; color: white; border: none; padding: 5px 10px; border-radius: 4px; cursor: pointer; }}
                a {{ color: #4299e1; text-decoration: none; }}
            </style>
        </head>
        <body>
            <nav>
                <div style="font-weight: bold; font-size: 20px;">SESH <span style="color: #4299e1;">ADMIN</span></div>
                <div>
                    <a href="/graphql" target="_blank" style="margin-right: 20px;">GraphQL</a>
                    <a href="/admin/logout" style="background: #e53e3e; color: white; padding: 5px 15px; border-radius: 4px;">Déconnexion</a>
                </div>
            </nav>
            <div class="container">
                <h1>Tricks en attente de validation</h1>
                <table>
                    <thead>
                        <tr>
                            <th>Date</th>
                            <th>Description</th>
                            <th>ID Spot</th>
                            <th>Actions</th>
                        </tr>
                    </thead>
                    <tbody>
                        {}
                    </tbody>
                </table>
            </div>
        </body>
        </html>
    "#,
        tricks_html
    ))
}

pub async fn trick_action_handler(
    State(state): State<Arc<AppState>>,
    Form(payload): Form<TrickActionForm>,
) -> impl IntoResponse {
    match payload.action.as_str() {
        "approve" => {
            let _ = TrickService::approve_trick(&state.pool, payload.trick_id).await;
        }
        "reject" => {
            let _ = TrickService::reject_trick(&state.pool, payload.trick_id).await;
        }
        _ => {}
    }
    Redirect::to("/admin")
}

pub async fn logout_handler(jar: CookieJar) -> impl IntoResponse {
    (
        jar.remove(Cookie::from("admin_token")),
        Redirect::to("/admin/login"),
    )
}
