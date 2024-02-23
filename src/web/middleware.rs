use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use tower_cookies::Cookies;

use crate::{error::Error, model::ModelManager};

async fn require_auth(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    mut req: Request,
    next: Next,
) -> Result<Response, Error> {
    let token = cookies
        .get("auth-token")
        .ok_or(Error::TokenNotFound)?
        .to_string();
    let token_parts: Vec<&str> = token.split(".").collect();

    let user = crate::model::user::UserModelControler::get_by_name(&mm, token_parts[0])
        .await?
        .ok_or(Error::UserNotFound)?;

    // TODO: validate exp-date

    if format!("{}:exp-date", user.username) != token_parts[2] {
        return Err(Error::FailedToAuthenticate);
    }

    let ctx = crate::context::Ctx {
        username: user.username,
    };

    req.extensions_mut().insert(ctx);

    Ok(next.run(req).await)
}
