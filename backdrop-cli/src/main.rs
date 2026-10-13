use backdrop_api::auth::*;

use std::collections::HashMap;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use url::Url;

const REDIRECT_URI: &str = "http://127.0.0.1:8080/callback";

#[tokio::main]
async fn main() {
    let auth_params = AuthRequest {
        client_id: UNSPLASH_CLIENT_ID,
        redirect_uri: REDIRECT_URI,
        response_type: "code",
        scope: "public",
    };

    let mut auth_url = Url::parse(UNSPLASH_AUTH_URI).unwrap();
    auth_url.set_query(Some(&serde_urlencoded::to_string(&auth_params).unwrap()));

    open::that(auth_url.as_str()).unwrap();

    let callback = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let (mut socket, _addr) = callback.accept().await.unwrap();
    let mut buffer = [0u8; 1024];
    let bytes_read = socket.read(&mut buffer).await.unwrap();

    let req_str = unsafe { std::str::from_utf8_unchecked(&buffer[..bytes_read]) };
    let req_line = req_str.lines().next().unwrap();

    let path_and_query = req_line.split_whitespace().nth(1).unwrap();
    let full_url = Url::parse(&format!("http://127.0.0.1:8080{path_and_query}")).unwrap();

    let params: HashMap<_, _> = full_url.query_pairs().into_owned().collect();

    let code = params.get("code").cloned().unwrap_or_default();

    let exchange_params = TokenRequest {
        redirect_uri: REDIRECT_URI,
        code: &code,
    };

    let token = reqwest::Client::builder()
        .user_agent("backdrop/1.0")
        .build()
        .unwrap()
        .post(BACKDROP_AUTH_URI)
        .json(&exchange_params)
        .send()
        .await
        .unwrap()
        .json::<TokenResponse>()
        .await
        .unwrap();

    println!("{:#?}", token);
}
