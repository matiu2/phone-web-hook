mod auth;
mod ntfy;

use auth::is_authed;
use worker::{Context, Env, Request, Response, Result, event};

#[event(fetch)]
pub async fn main(mut req: Request, env: Env, _ctx: Context) -> Result<Response> {
    if !is_authed(&req, &env) {
        return Response::error("Unauthorised", 401);
    }
    // Get the incoming body
    let body = req.text().await?;
    // Send it to ntfy
    ntfy::send(&body, &env).await?;
    Response::ok("ok")
}
