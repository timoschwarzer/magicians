use crate::game_logic::{CardColor, Game};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug)]
#[serde(tag = "type", rename_all_fields = "camelCase")]
pub enum WebSocketNotification {
    CurrentGame { game: Game },
    SetPlayerIndex { player_index: Option<usize> },
    SetPlayerKey { key: String },
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type", rename_all_fields = "camelCase")]
pub enum WebSocketCommand {
    Authenticate { key: String },
    Join { name: String },
    SignalReady { ready: bool },
    SetTrumpColor { trump_color: Option<CardColor> },
    SubmitPrediction { prediction: u8 },
    ChangePrediction { prediction: u8 },
    SubmitWinCount { win_count: u8 },
}
