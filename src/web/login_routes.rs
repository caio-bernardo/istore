use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use tracing::info;

use crate::{
    model::{self, user::User},
    AppState,
};

use super::model::ModelManager;
use super::Error;

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct SignUpPayload {
    username: String,
    password: String,
    confirm_password: String,
}

pub fn login_routes(state: AppState) -> Router {
    Router::new()
        .route("/login", post(login_handler))
        .route("/logout", post(logout_handler))
        .route("signup", post(signup_handler))
        .layer(CookieManagerLayer::new())
        .with_state(state)
}

async fn login_handler(
    cookies: Cookies,
    State(mm): State<ModelManager>,
    Json(payload): Json<LoginPayload>,
) -> Result<Response, Error> {
    let user: model::user::UserForLogin =
        model::user::UserModelControler::get_by_name(&mm, &payload.username)
            .await?
            .ok_or(Error::UserNotFound)?;
    if user.password != payload.password {
        return Err(Error::PasswordsDontMatch);
    }

    let token = format!("{0}.{1}.{0}:{1}", payload.username, "exp-date");

    cookies.add(Cookie::new("auth-token", token));

    Ok(Json(json!({"result": "success"})).into_response())
}

async fn logout_handler(cookies: Cookies) -> impl IntoResponse {
    cookies.remove(Cookie::new("auth-token", ""));

    (StatusCode::OK, Json(json!({"result": "success"}))).into_response()
}

async fn signup_handler(
    cookies: Cookies,
    State(mm): State<ModelManager>,
    Json(payload): Json<SignUpPayload>,
) -> Result<Response, Error> {
    if payload.password != payload.confirm_password {
        return Err(Error::PasswordsDontMatch);
    }

    let user_fc = model::user::UserForCreate::new(payload.username, payload.password);
    let user_id = model::user::UserModelControler::create(&mm, user_fc)
        .await
        .map_err(|_| Error::FailedToCreate)?; // TODO: map the reason the creation failed

    Ok((StatusCode::CREATED, Json(json!({"user_id": user_id}))).into_response())
}
