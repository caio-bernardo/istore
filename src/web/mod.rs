mod api_routes;
mod login_routes;
mod middleware;

use crate::error::Error;
use crate::model::{self, ModelManager};
use axum::Router;

pub fn routes(state: ModelManager) -> Router {
    Router::new().merge(login_routes::login_routes(state.clone()))
}
