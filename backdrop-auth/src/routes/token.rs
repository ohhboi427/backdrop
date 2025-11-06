use crate::error::Error;
use crate::state::AppState;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use std::collections::HashMap;
use std::sync::Arc;

pub async fn token(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    use axum::Json;
    use unsplash_api::auth::AuthToken;
    use uuid::Uuid;

    let session_id = Uuid::parse_str(params.get("state").ok_or(Error::MissingParam)?)
        .map_err(|_| Error::InvalidResponse)?;

    let mut sessions = state.sessions.write().await;
    match sessions.get(&session_id) {
        Some((Some(_), _)) => {
            let (response, _) = sessions.remove(&session_id).unwrap();

            Ok(Json(AuthToken::Bearer(response.unwrap().access_token)))
        }
        Some((None, _)) => Err(Error::TokenNotReady),
        None => Err(Error::InvalidSession),
    }
}
