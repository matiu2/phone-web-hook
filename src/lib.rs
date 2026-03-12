use worker::{Context, Env, Request, Response, Result, event};

#[event(fetch)]
pub async fn main(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    Response::ok("ok")
}
