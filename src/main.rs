#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("UNSPLASH_API_KEY").unwrap();

    let auth = format!("Client-ID {}", api_key);
    let mut auth = reqwest::header::HeaderValue::from_str(&auth).unwrap();
    auth.set_sensitive(true);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::AUTHORIZATION, auth);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let response = client
        .get("https://api.unsplash.com/photos/random")
        .send()
        .await
        .unwrap();

    println!("{:?}", response);
}
