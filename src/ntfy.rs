use worker::{Env, Fetch, Method, Request, RequestInit, Result, console_error};

/// Send the event body to ntfy.sh
pub async fn send(body: &str, env: &Env) -> Result<()> {
    let Ok(topic) = env.var("NTFY_TOPIC").map(|topic| topic.to_string()) else {
        console_error!("No NTFY_TOPIC env var found");
        return Err("Internal Error".into());
    };
    let url = format!("https://ntfy.sh/{topic}");
    let mut init = RequestInit::new();
    init.with_method(Method::Post);
    init.with_body(Some(body.into()));
    let req = Request::new_with_init(&url, &init)?;
    let response = Fetch::Request(req).send().await?;
    let status_code = response.status_code();
    if !(200..300).contains(&status_code) {
        console_error!("ntfy.sh returned error status code: {status_code}");
        Err("ntfy request failed".into())
    } else {
        Ok(())
    }
}
