use crate::protocol::WebSocketNotification;
use crate::timestamp::get_unix_timestamp;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::num::NonZeroU8;
use std::ops::Sub;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use tokio::sync::broadcast::Sender;
use uuid::Uuid;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    /** Player key; used when reconnecting to a game that is in progress */
    #[serde(skip)]
    key: String,
    /** The player's name */
    name: String,
    /** The player's current prediction, or None if we are not currently in a round */
    prediction: Option<u8>,
    /** The player's points */
    points: i32,
    /** Points gained (or lost when negative) in the last round */
    points_delta: Option<i32>,
    /** Whether this player is ready to continue */
    ready_to_continue: bool,
}

#[derive(Serialize, Clone, Debug)]
#[serde(tag = "type", rename_all_fields = "camelCase")]
pub enum GameState {
    /// Setup state where players can still join
    Setup,
    /// Idle state, here players can review the leaderboard and deal cards
    Idle {
        /// The game round
        round: NonZeroU8,
        /// The index of the player who starts predicting and playing
        first_player_index: usize,
        /// The index of the player who predicts last
        last_player_index: usize,
    },
    /// Predicting state where players submit predictions one after another
    Predicting {
        /// The game round
        round: NonZeroU8,
        /// The index of the player who is currently asked to submit their prediction
        player_index: usize,
        /// The index of the player who starts predicting and playing
        first_player_index: usize,
        /// The index of the player who predicts last
        last_player_index: usize,
        /// The value of the prediction which is not allowed to be submitted, or None if
        /// all predictions are allowed
        disallowed_prediction: Option<u8>,
    },
    /// State during which the players play all their cards
    Playing {
        /// The game round
        round: NonZeroU8,
        first_player_index: usize,
    },
    /// State where players submit their win counts
    DistributingPoints {
        /// The game round
        round: NonZeroU8,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CardColor {
    Green,
    Blue,
    Yellow,
    Red,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    /// Unix timestamp of when this game was created; used for pruning old games
    created_at: f64,
    /// The list of players
    players: Vec<Player>,
    /// Current trump color, or None if there is no trump color
    trump_color: Option<CardColor>,
    /// Current game state
    state: GameState,

    #[serde(skip)]
    broadcast_sender: Sender<WebSocketNotification>,
}

#[derive(Serialize, Clone, Debug)]
pub enum GameLogicError {
    GameAlreadyInProgress,
    NotPossibleInCurrentGameState,
    InvalidInputNumber,
}

impl Game {
    /// Generate a new unique player key
    fn generate_player_key(&self) -> String {
        loop {
            let key = Uuid::new_v4().to_string();
            if !self.players.iter().any(|p| p.key == key) {
                return key;
            }
        }
    }

    pub fn new() -> Self {
        Self {
            created_at: get_unix_timestamp(),
            players: vec![],
            broadcast_sender: Sender::new(16),
            trump_color: None,
            state: GameState::Setup,
        }
    }

    pub fn broadcast_sender(&self) -> &Sender<WebSocketNotification> {
        &self.broadcast_sender
    }

    /// Returns whether this game is in progress
    pub fn has_started(&self) -> bool {
        !matches!(self.state, GameState::Setup)
    }

    /// Add a player to this game with a given name
    pub fn add_player<S: Into<String>>(
        &mut self,
        name: S,
    ) -> Result<(usize, String), GameLogicError> {
        if self.has_started() {
            return Err(GameLogicError::GameAlreadyInProgress);
        }

        let key = self.generate_player_key();

        self.players.push(Player {
            key: key.clone(),
            name: name.into(),
            prediction: None,
            points: 0,
            points_delta: None,
            ready_to_continue: false,
        });

        Ok((self.players.len() - 1, key))
    }

    /// Returns the player index of a player with a given key, or None if none was found
    pub fn get_player_index_by_key(&self, key: &str) -> Option<usize> {
        self.players.iter().position(|p| p.key == key)
    }

    /// Returns the index of the player who gets to predict and play first
    fn get_first_player_index(&self, round: NonZeroU8) -> usize {
        (round.get() as usize - 1) % self.players.len()
    }

    /// Returns the index of the player who gets to predict last
    fn get_last_player_index(&self, round: NonZeroU8) -> usize {
        (round.get() as isize - 2).rem_euclid(self.players.len() as isize) as usize
    }

    /// Submit a prediction for a player
    pub fn submit_prediction(
        &mut self,
        player_index: usize,
        prediction: u8,
    ) -> Result<(), GameLogicError> {
        let GameState::Predicting {
            player_index: current_player_index,
            last_player_index,
            first_player_index,
            round,
            disallowed_prediction,
            ..
        } = self.state
        else {
            return Err(GameLogicError::NotPossibleInCurrentGameState);
        };

        if player_index != current_player_index {
            return Err(GameLogicError::NotPossibleInCurrentGameState);
        }

        if prediction > round.get() {
            return Err(GameLogicError::InvalidInputNumber);
        }

        if disallowed_prediction
            .is_some_and(|disallowed_prediction| prediction == disallowed_prediction)
        {
            return Err(GameLogicError::InvalidInputNumber);
        }

        self.players[player_index].prediction = Some(prediction);
        self.players[player_index].ready_to_continue = true;

        if !self.continue_if_ready()? {
            let next_player_index = (player_index + 1) % self.players.len();
            let sum_of_all_predictions: u8 =
                self.players.iter().map(|p| p.prediction.unwrap_or(0)).sum();

            self.state = GameState::Predicting {
                round,
                player_index: next_player_index,
                first_player_index,
                last_player_index,
                // Make sure the last player cannot predict an amount that matches the total amount
                disallowed_prediction: if next_player_index == last_player_index
                    && sum_of_all_predictions <= round.get()
                {
                    Some(round.get() - sum_of_all_predictions)
                } else {
                    None
                },
            };
        }

        Ok(())
    }

    /// Change the prediction of a player during the game phase
    pub fn change_prediction(
        &mut self,
        player_index: usize,
        prediction: u8,
    ) -> Result<(), GameLogicError> {
        if !matches!(self.state, GameState::Playing { .. }) {
            return Err(GameLogicError::NotPossibleInCurrentGameState);
        }

        let Some(player) = self.players.get_mut(player_index) else {
            return Err(GameLogicError::InvalidInputNumber);
        };

        player.prediction = Some(prediction);

        Ok(())
    }

    /// Sets the trump color of the game
    pub fn set_trump_color(
        &mut self,
        trump_color: Option<CardColor>,
    ) -> Result<(), GameLogicError> {
        if !matches!(
            self.state,
            GameState::Idle { .. } | GameState::Predicting { .. } | GameState::Playing { .. }
        ) {
            return Err(GameLogicError::NotPossibleInCurrentGameState);
        }

        self.trump_color = trump_color;

        Ok(())
    }

    /**
     * Continue the state machine if the current state is ready. The ready conditions vary between
     * game states.
     */
    fn continue_if_ready(&mut self) -> Result<bool, GameLogicError> {
        macro_rules! require_all_players_ready {
            () => {
                if !self.players.iter().all(|p| p.ready_to_continue) {
                    return Ok(false);
                }
            };
        }

        macro_rules! require_one_player_ready {
            ($player_index:expr) => {
                if !self.players.get($player_index).map(|p| p.ready_to_continue).unwrap_or(false) {
                    return Ok(false);
                }
            };
        }

        match self.state {
            GameState::Setup => {
                require_all_players_ready!();
                self.state = GameState::Idle {
                    round: NonZeroU8::new(1).unwrap(),
                    first_player_index: self.get_first_player_index(NonZeroU8::new(1).unwrap()),
                    last_player_index: self.get_last_player_index(NonZeroU8::new(1).unwrap()),
                };
            }
            GameState::Idle {
                round,
                first_player_index,
                last_player_index,
                ..
            } => {
                require_one_player_ready!(last_player_index);
                self.state = GameState::Predicting {
                    round,
                    player_index: first_player_index,
                    first_player_index,
                    last_player_index,
                    disallowed_prediction: None,
                };
            }
            GameState::Predicting { round, first_player_index, .. } => {
                require_all_players_ready!();
                self.state = GameState::Playing { round, first_player_index };
            }
            GameState::Playing { round, .. } => {
                require_all_players_ready!();
                self.trump_color = None;
                self.players.iter_mut().for_each(|p| p.points_delta = None);
                self.state = GameState::DistributingPoints { round };
            }
            GameState::DistributingPoints { round } => {
                require_all_players_ready!();
                self.players.iter_mut().for_each(|p| p.prediction = None);

                let next_round = round.saturating_add(1);
                self.state = GameState::Idle {
                    round: next_round
                        .try_into()
                        .map_err(|_| GameLogicError::InvalidInputNumber)?,
                    first_player_index: self.get_first_player_index(next_round),
                    last_player_index: self.get_last_player_index(next_round),
                };
            }
        };

        self.players
            .iter_mut()
            .for_each(|p| p.ready_to_continue = false);

        Ok(true)
    }

    /// Submit the win count for a player
    pub fn submit_win_count(
        &mut self,
        player_index: usize,
        win_count: u8,
    ) -> Result<(), GameLogicError> {
        if !matches!(self.state, GameState::DistributingPoints { .. }) {
            return Err(GameLogicError::NotPossibleInCurrentGameState);
        }

        let Some(player) = self.players.get_mut(player_index) else {
            return Err(GameLogicError::InvalidInputNumber);
        };

        if player.ready_to_continue {
            return Err(GameLogicError::NotPossibleInCurrentGameState);
        };

        let points_delta = if win_count == player.prediction.unwrap_or(0) {
            20 + player.prediction.unwrap_or(0) as i32 * 10
        } else {
            win_count.abs_diff(player.prediction.unwrap_or(0)) as i32 * -10
        };

        player.points_delta = Some(points_delta);
        player.points += points_delta;

        player.ready_to_continue = true;
        self.continue_if_ready()?;

        Ok(())
    }

    /// Mark a player as being ready or not ready
    pub fn signal_player_ready(
        &mut self,
        player_index: usize,
        ready: bool,
    ) -> Result<(), GameLogicError> {
        if !matches!(
            self.state,
            GameState::Setup | GameState::Playing { .. } | GameState::Idle { .. }
        ) {
            return Err(GameLogicError::NotPossibleInCurrentGameState);
        }

        let Some(player) = self.players.get_mut(player_index) else {
            return Err(GameLogicError::InvalidInputNumber);
        };

        player.ready_to_continue = ready;
        self.continue_if_ready()?;

        Ok(())
    }

    pub fn broadcast_state(&self) {
        if let Err(error) = self
            .broadcast_sender
            .send(WebSocketNotification::CurrentGame { game: self.clone() })
        {
            tracing::warn!("Failed to broadcast game state state: {}", error);
        }
    }
}

#[derive(Clone)]
pub struct ApplicationState {
    pub games: Arc<Mutex<HashMap<Uuid, Arc<Mutex<Game>>>>>, // ID → Game
}

impl ApplicationState {
    /// Clean up games that are more than 14 days old
    pub async fn clean_old_games(&self) {
        let unix_time_2_weeks_ago = SystemTime::now()
            .sub(Duration::from_hours(24 * 14))
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        tracing::debug!("Cleaning old games before: {}", unix_time_2_weeks_ago);

        let games = self.games.clone();
        let game_count_before = games.lock().await.len();

        let game_count_after = tokio::task::spawn_blocking(move || {
            let mut games = games.blocking_lock();
            games.retain(|_, game| game.blocking_lock().created_at > unix_time_2_weeks_ago);
            games.len()
        })
        .await
        .expect("Error executing cleanup task");

        tracing::info!("Purged games: {}, active games: {}", game_count_before - game_count_after, game_count_after);
    }
}
