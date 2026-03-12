use serde::Deserialize;
use worker::{Env, Request, console_error, console_log};

/// Call to check
pub fn is_authed(req: &Request, env: &Env) -> bool {
    let Ok(expected) = env.secret("SECRET_TOKEN").map(|token| token.to_string()) else {
        console_error!("No SECRET_TOKEN found in env vars");
        return false;
    };
    let got: Query = match req.query() {
        Ok(query) => query,
        Err(err) => {
            console_log!("Unable to parse query: {err:?}");
            return false;
        }
    };
    match (&expected, &got.token) {
        // No token in params
        (_, None) => {
            console_log!("No `token` found in http params");
            false
        }
        // Good auth
        (expected, Some(got)) if expected == got => true,
        // Token, but doesn't match expected
        _ => {
            console_log!("Token didn't match expected token");
            false
        }
    }
}

/// Just to parse the http args, eg. ...?token=abcd
#[derive(Deserialize)]
struct Query {
    token: Option<String>,
}
