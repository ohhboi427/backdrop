use worker::*;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct TokenPayload {
    redirect_uri: String,
    code: String,
}

#[derive(Serialize)]
struct TokenRequest<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    redirect_uri: &'a str,
    code: &'a str,
    grant_type: &'a str,
}

async fn unsplash_token_exchange(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let payload = match req.json::<TokenPayload>().await {
        Ok(payload) =>payload,
        Err(e) => return Response::error(e.to_string(), 400),
    };

    let client_id = ctx.var("UNSPLASH_CLIENT_ID")?.to_string();
    let client_secret = ctx.secret("UNSPLASH_CLIENT_SECRET")?.to_string();

    let params = TokenRequest {
        client_id: &client_id,
        client_secret: &client_secret,
        redirect_uri: &payload.redirect_uri,
        code: &payload.code,
        grant_type: "authorization_code",
    };

    let body = match serde_urlencoded::to_string(&params) {
        Ok(body) => body,
        Err(e) => return Response::error(e.to_string(), 500),
    };

    let headers = Headers::new();
    headers.set("Content-Type", "application/x-www-form-urlencoded")?;
    headers.set("Accept", "application/json")?;
    headers.set("User-Agent", "backdrop/1.0")?;

    let mut init = RequestInit::new();
    init.with_method(Method::Post);
    init.with_headers(headers);
    init.with_body(Some(body.into()));

    let request = Request::new_with_init("https://unsplash.com/oauth/token", &init)?;
    let response = Fetch::Request(request).send().await?;

    let body = response.body();
    let status = response.status_code();

    let headers = Headers::new();
    headers.set("Access-Control-Allow-Origin", "*")?;
    headers.set("Content-Type", "application/json")?;

    Ok(Response::from_body(body.clone())?
        .with_status(status)
        .with_headers(headers))
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .post_async("/exchange", unsplash_token_exchange)
        .options("/exchange", |_req, _ctx| {
            let headers = Headers::new();
            headers.set("Access-Control-Allow-Origin", "*")?;
            headers.set("Access-Control-Allow-Methods", "POST, OPTIONS")?;
            headers.set("Access-Control-Allow-Headers", "Content-Type")?;

            Ok(Response::empty()?.with_headers(headers))
        })
        .run(req, env)
        .await
}
