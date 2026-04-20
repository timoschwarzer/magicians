use std::sync::Arc;
use crate::game_logic::{ApplicationState, Game};
use axum::Json;
use axum::extract::State;
use serde::Serialize;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGameResponse {
    game_id: Uuid,
}

pub async fn handler(State(state): State<ApplicationState>) -> Json<CreateGameResponse> {
    let uuid = Uuid::new_v4();

    state
        .games
        .lock()
        .await
        .insert(uuid.clone(), Arc::new(Mutex::new(Game::new())));

    Json(CreateGameResponse {
        game_id: uuid,
    })
}
