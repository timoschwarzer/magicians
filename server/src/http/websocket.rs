use crate::protocol::{WebSocketCommand, WebSocketNotification};
use crate::game_logic::{ApplicationState, Game, GameLogicError};
use axum::extract::ws::CloseFrame;
use axum::extract::{Path, State, WebSocketUpgrade, ws};
use axum::http::Version;
use axum::response::Response;
use std::ops::DerefMut;
use serde::Serialize;
use uuid::Uuid;

fn handle_game_command(
    game: &mut Game,
    player_index: usize,
    command: WebSocketCommand,
) -> Result<(), GameLogicError> {
    match command {
        WebSocketCommand::SignalReady { ready } => {
            game.signal_player_ready(player_index, ready)?;
            game.broadcast_state();
        }
        WebSocketCommand::SetTrumpColor { trump_color } => {
            game.set_trump_color(trump_color)?;
            game.broadcast_state();
        }
        WebSocketCommand::SubmitPrediction { prediction } => {
            game.submit_prediction(player_index, prediction)?;
            game.broadcast_state();
        }
        WebSocketCommand::ChangePrediction { prediction } => {
            game.change_prediction(player_index, prediction)?;
            game.broadcast_state();
        }
        WebSocketCommand::SubmitWinCount { win_count } => {
            game.submit_win_count(player_index, win_count)?;
            game.broadcast_state();
        }
        _ => {}
    }

    Ok(())
}


#[derive(Serialize)]
#[serde(rename_all_fields = "camelCase")]
enum AuthenticationCommandResult {
    MaybeAuthenticated,
    Joined {
        key: String,
    },
}

fn handle_authentication_command(
    game: &mut Game,
    player_index: &mut Option<usize>,
    command: WebSocketCommand,
) -> Result<AuthenticationCommandResult, GameLogicError> {
    match command {
        WebSocketCommand::Authenticate { key } => {
            *player_index = game.get_player_index_by_key(&key);
        }
        WebSocketCommand::Join { name } => {
            let (index, key) = game.add_player(name)?;
            *player_index = Some(index);
            game.broadcast_state();
            return Ok(AuthenticationCommandResult::Joined {key});
        }
        _ => {}
    }

    Ok(AuthenticationCommandResult::MaybeAuthenticated)
}

pub async fn handler(
    ws: WebSocketUpgrade,
    version: Version,
    State(state): State<ApplicationState>,
    Path(game_id): Path<Uuid>,
) -> Response {
    tracing::debug!("Accepted a WebSocket using {version:?}");

    let Some(game) = state.games.lock().await.get(&game_id).cloned() else {
        return ws.on_upgrade(|mut ws| async move {
            if let Err(e) = ws
                .send(ws::Message::Close(Some(CloseFrame {
                    code: 4404,
                    reason: "".into(),
                })))
                .await
            {
                tracing::debug!("Client disconnected while sending close frame: {e}");
            }
        });
    };

    let mut broadcast_receiver = game.lock().await.broadcast_sender().subscribe();

    ws.on_upgrade(|mut ws| async move {
        let game = game.clone();

        if let Err(e) = ws.send(ws::Message::Text(serde_json::to_string(&WebSocketNotification::CurrentGame {
            game: game.lock().await.clone(),
        }).unwrap().into())).await {
            tracing::debug!("Client disconnected abruptly: {e}");
            return;
        }

        let mut player_index: Option<usize> = None;

        loop {
            tokio::select! {
                res = ws.recv() => {
                    match res {
                        Some(Ok(ws::Message::Text(message))) => 'command: {
                            let Ok(command) = serde_json::from_str::<WebSocketCommand>(message.as_str()) else {
                                break 'command;
                            };

                            let mut game_mutex_guard = game.lock().await;
                            let game = game_mutex_guard.deref_mut();

                            let result = if let Some(player_index) = player_index {
                                handle_game_command(game, player_index, command)
                            } else {
                                let authentication_result = handle_authentication_command(game, &mut player_index, command);

                                if let Ok(AuthenticationCommandResult::Joined { key }) = &authentication_result {
                                    if let Err(e) = ws.send(ws::Message::Text(serde_json::to_string(&WebSocketNotification::SetPlayerKey {
                                        key: key.clone(),
                                    }).unwrap().into())).await {
                                        tracing::debug!("Client disconnected abruptly: {e}");
                                    }
                                }

                                if authentication_result.is_ok() {
                                    if let Err(e) = ws.send(ws::Message::Text(serde_json::to_string(&WebSocketNotification::SetPlayerIndex {
                                        player_index,
                                    }).unwrap().into())).await {
                                        tracing::debug!("Client disconnected abruptly: {e}");
                                    }
                                }

                                authentication_result.map(|_| {})
                            };

                            if let Err(err) = result {
                                tracing::error!("Error while handling command: {:?}", err);
                            }
                        }
                        Some(Ok(_)) => {}
                        Some(Err(e)) => tracing::debug!("Client disconnected abruptly: {e}"),
                        None => break,
                    }
                }

                res = broadcast_receiver.recv() => {
                    match res {
                        Ok(ref msg) => if let Err(e) = ws.send(ws::Message::Text(serde_json::to_string(&msg).unwrap().into())).await {
                            tracing::debug!("Client disconnected abruptly: {e}");
                        }
                        Err(_) => continue,
                    }
                }
            }
        }
    })
}
