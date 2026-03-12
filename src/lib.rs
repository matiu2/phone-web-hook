pub mod auth;

use auth::is_authed;
use worker::{Context, Env, Request, Response, Result, event};

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    if !is_authed(&req, &env) {
        return Response::error("Unauthorised", 401);
    }
    Response::ok("ok")
}
