use crate::state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use std::sync::Arc;

const UNSPLASH_AUTH_URL: &'static str = "https://unsplash.com/oauth/authorize";

pub async fn auth(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    use axum::Json;
    use std::time::Instant;
    use unsplash_api::auth::AuthResponse;
    use url::Url;
    use uuid::Uuid;

    let session_id = Uuid::new_v4();

    let redirect_url = state.server_url.join("/exchange").unwrap();
    let auth_url = Url::parse_with_params(
        UNSPLASH_AUTH_URL,
        [
            ("client_id", state.access_key.as_str()),
            ("redirect_uri", redirect_url.as_str()),
            ("response_type", "code"),
            ("scope", "public"),
            ("state", session_id.to_string().as_str()),
        ],
    )
    .unwrap();

    state
        .sessions
        .write()
        .await
        .insert(session_id, (None, Instant::now()));

    Json(AuthResponse {
        session_id,
        auth_url,
    })
}
