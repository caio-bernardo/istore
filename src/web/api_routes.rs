use axum::{
    extract::{Json, Path, State},
    response::Response,
    routing::{get, post},
    Extension, Router,
};

use crate::{context::Ctx, error::Error, model, AppState};

pub fn api_routes(state: AppState) -> Router {
    Router::new()
        .route(
            "/:id",
            get(get_task_handler)
                .put(update_task_handler)
                .delete(delete_task_handler),
        )
        .route("/", post(create_task_handler))
        .with_state(state)
}

async fn get_task_handler(
    Extension(ctx): Extension<Ctx>,
    State(mm): State<model::ModelManager>,
) -> Result<Response, Error> {
    todo!()
}

async fn create_task_handler(
    Extension(ctx): Extension<Ctx>,
    State(mm): State<model::ModelManager>,
    Json(payload): Json<model::task::TaskForCreate>,
) -> Result<Response, Error> {
    todo!();
}

async fn delete_task_handler(
    Extension(ctx): Extension<Ctx>,
    State(mm): State<model::ModelManager>,
    Path(task_id): Path<i64>,
) -> Result<Response, Error> {
    todo!();
}

async fn update_task_handler(
    Extension(ctx): Extension<Ctx>,
    State(mm): State<model::ModelManager>,
    Path(task_id): Path<i64>,
    Json(payload): Json<model::task::TaskForUpdate>,
) -> Result<Response, Error> {
    todo!();
}
