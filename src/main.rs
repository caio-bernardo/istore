#![allow(unused)]
// TODO: #![forbid(clippy::unwrap_used)]

mod context;
mod error;
mod model;
mod web;

use axum::{
    extract::{FromRef, Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, RequestExt, Router,
};
use serde::Deserialize;
use serde_json::json;
use sqlx::{prelude::FromRow, Pool, Sqlite, SqlitePool};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use tower_http::trace::TraceLayer;
use tracing::info;

const ADDRESS: &str = "127.0.0.1:3030";
const SESSION_ID: &str = "session-id";

#[derive(Clone, Debug, FromRef)]
struct AppState {
    mm: model::ModelManager,
}

impl AppState {
    async fn new() -> Self {
        AppState {
            mm: model::ModelManager::new().await,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let state = AppState::new().await;

    let app = Router::new()
        .merge(web::routes(state))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(ADDRESS).await?;
    tracing::info!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
