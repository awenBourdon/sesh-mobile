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
            <title>SESH ADMIN - LOGIN</title>
            <style>
                body {
                    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                    margin: 0;
                    background: #F7F7F7;
                    color: #1A1A1A;
                }
                form {
                    background: white;
                    padding: 3rem;
                    border-radius: 24px;
                    border: 1px solid rgba(0,0,0,0.1);
                    width: 320px;
                }
                .logo {
                    font-size: 32px;
                    font-weight: 900;
                    text-align: center;
                    letter-spacing: -1.5px;
                    margin-bottom: 2rem;
                }
                .field { margin-bottom: 1.5rem; }
                label {
                    display: block;
                    margin-bottom: 0.5rem;
                    font-size: 11px;
                    font-weight: 900;
                    color: rgba(0,0,0,0.4);
                }
                input {
                    width: 100%;
                    padding: 0.8rem;
                    background: #F9F9F9;
                    border: 1px solid rgba(0,0,0,0.05);
                    border-radius: 12px;
                    box-sizing: border-box;
                    font-size: 14px;
                    outline: none;
                }
                input:focus {
                    border-color: #1A1A1A;
                }
                button {
                    width: 100%;
                    padding: 1rem;
                    background: #1A1A1A;
                    color: white;
                    border: none;
                    border-radius: 20px;
                    cursor: pointer;
                    font-weight: 900;
                    font-size: 14px;
                    margin-top: 1rem;
                }
                button:hover { background: #000; }
            </style>
        </head>
        <body>
            <form action="/admin/login" method="POST">
                <div class="logo">SESH</div>
                <div class="field">
                    <label>EMAIL</label>
                    <input type="email" name="email" required placeholder="name@example.com">
                </div>
                <div class="field">
                    <label>PASSWORD</label>
                    <input type="password" name="password" required>
                </div>
                <button type="submit">SE CONNECTER</button>
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
        tricks_html = "<tr><td colspan='4' style='text-align:center; padding: 40px; color: rgba(0,0,0,0.3); font-size: 14px;'>Aucun trick en attente de validation.</td></tr>".to_string();
    } else {
        for trick in pending_tricks {
            tricks_html.push_str(&format!(
                r#"
                <tr>
                    <td class="date">{}</td>
                    <td class="desc">{}</td>
                    <td class="id">{}</td>
                    <td class="actions">
                        <form action="/admin/trick-action" method="POST" style="display:inline;">
                            <input type="hidden" name="trick_id" value="{}">
                            <button type="submit" name="action" value="approve" class="btn-approve">APPROUVER</button>
                            <button type="submit" name="action" value="reject" class="btn-reject">REJETER</button>
                        </form>
                    </td>
                </tr>
                "#,
                trick.created_at.unwrap_or_default().format("%d.%m.%y"),
                trick.description.as_deref().unwrap_or("SANS DESCRIPTION").to_uppercase(),
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
            <title>SESH ADMIN - DASHBOARD</title>
            <style>
                body {{
                    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
                    margin: 0;
                    background: #F7F7F7;
                    color: #1A1A1A;
                }}
                nav {{
                    background: white;
                    padding: 1.5rem 3rem;
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    border-bottom: 1px solid rgba(0,0,0,0.05);
                }}
                .logo {{ font-weight: 900; font-size: 24px; letter-spacing: -1px; }}
                .container {{ padding: 3rem; max-width: 1100px; margin: 0 auto; }}
                h1 {{ font-weight: 900; font-size: 32px; letter-spacing: -1px; margin-bottom: 2rem; }}

                table {{
                    width: 100%;
                    border-collapse: collapse;
                    background: white;
                    border-radius: 24px;
                    overflow: hidden;
                    border: 1px solid rgba(0,0,0,0.05);
                }}
                th, td {{ padding: 20px 24px; text-align: left; }}
                th {{
                    background: #F9F9F9;
                    color: rgba(0,0,0,0.4);
                    text-transform: uppercase;
                    font-size: 11px;
                    font-weight: 900;
                    letter-spacing: 0.5px;
                }}
                tr:not(:last-child) td {{ border-bottom: 1px solid rgba(0,0,0,0.03); }}

                .desc {{ font-weight: 800; font-size: 15px; }}
                .date, .id {{ font-family: monospace; font-size: 13px; color: rgba(0,0,0,0.5); }}

                .btn-approve {{
                    background: #1A1A1A;
                    color: white;
                    border: none;
                    padding: 8px 16px;
                    border-radius: 12px;
                    cursor: pointer;
                    font-weight: 900;
                    font-size: 11px;
                }}
                .btn-reject {{
                    background: white;
                    color: rgba(0,0,0,0.3);
                    border: 1px solid rgba(0,0,0,0.1);
                    padding: 8px 16px;
                    border-radius: 12px;
                    cursor: pointer;
                    font-weight: 900;
                    font-size: 11px;
                    margin-left: 8px;
                }}
                .btn-reject:hover {{ color: #FF4D4D; border-color: #FF4D4D; }}

                .nav-links a {{
                    color: #1A1A1A;
                    text-decoration: none;
                    font-weight: 800;
                    font-size: 13px;
                    margin-right: 2rem;
                }}
                .btn-logout {{
                    color: rgba(0,0,0,0.4) !important;
                    font-weight: 700 !important;
                }}
            </style>
        </head>
        <body>
            <nav>
                <div class="logo">SESH <span style="color: rgba(0,0,0,0.2)">ADMIN</span></div>
                <div class="nav-links">
                    <a href="/graphql" target="_blank">GRAPHQL</a>
                    <a href="/admin/logout" class="btn-logout">DÉCONNEXION</a>
                </div>
            </nav>
            <div class="container">
                <h1>MODÉRATION DES TRICKS</h1>
                <table>
                    <thead>
                        <tr>
                            <th>DATE</th>
                            <th>DESCRIPTION</th>
                            <th>ID SPOT</th>
                            <th>ACTIONS</th>
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
