use std::collections::HashMap;

use crate::service_intermediary::GameFetchWarning;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameJson {
    pub clock: Option<Clock>,
    pub clocks: Option<Vec<i64>>,
    pub created_at: Option<u64>,
    pub id: Option<String>,
    pub last_move_at: Option<u64>,
    pub moves: Option<String>,
    pub perf: Option<String>,
    pub players: Option<Players>,
    pub rated: Option<bool>,
    pub speed: Option<String>,
    pub status: Option<String>, // draw, mate, resign
    pub variant: Option<String>,
    pub winner: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clock {
    pub increment: Option<i32>,
    pub initial: Option<i32>,
    pub total_time: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Players {
    pub black: Option<PlayerDetail>,
    pub white: Option<PlayerDetail>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerDetail {
    pub rating: Option<i32>,
    pub rating_diff: Option<i32>,
    pub user: Option<User>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<String>,
    pub name: Option<String>,
}

pub fn convert_games_with_errors_to_displayable_format(
    games_with_errors: HashMap<usize, GameFetchWarning>,
) -> Vec<(usize, String)> {
    if games_with_errors.is_empty() {
        return Vec::new();
    }

    let mut enum_conversion_map: HashMap<GameFetchWarning, String> = HashMap::new();
    enum_conversion_map.insert(
        GameFetchWarning::GameHasNotEnoughMoves,
        String::from("Game does not have enough moves."),
    );
    enum_conversion_map.insert(
        GameFetchWarning::InternalErrorOccuredWhileProcessingAGame,
        String::from("An internal error occured while processing this game."),
    );

    let mut converted_errors = games_with_errors
        .into_iter()
        .map(|(i, warning_enum)| {
            (
                i + 1, // Initially we enter the game index, we wish to display in non-indexed format.
                enum_conversion_map
                    .get(&warning_enum)
                    .expect("Warning enum not found")
                    .to_string(),
            )
        })
        .collect::<Vec<(usize, String)>>();

    // Sanity check
    assert!(
        !converted_errors.iter().any(|(i, _)| *i == 0),
        "Game index should start from 1."
    );

    converted_errors.sort_by_key(|k| k.0.clone());
    converted_errors
}
