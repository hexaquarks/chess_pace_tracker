use crate::api::{ChessDataRequest, ChessDataResponse};
use actix_web::Error;
use futures_util::StreamExt;
use reqwest;
use serde_json::{self};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub status: Option<String>,
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

pub async fn fetch_lichess_player_data(
    request_data: ChessDataRequest,
) -> Result<ChessDataResponse, Error> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://lichess.org/api/games/user/{}?max={}&rated=true&clocks=true",
        request_data.username, request_data.games_count,
    );

    let res = client
        .get(&url)
        .header("Accept", "application/x-ndjson")
        .send()
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Request failed: {:?}", e))
        })?;

    if !res.status().is_success() {
        return Err(Error::from(actix_web::error::ErrorInternalServerError(
            "Lichess API returned non-success status",
        )));
    }

    let stream = res.bytes_stream();
    stream
        .for_each(|game_result| async {
            match game_result {
                Ok(game) => {
                    let a: GameJson =
                        serde_json::from_slice(&game).expect("Failed to serialize GameJson");

                    println!("moves: {}", a.moves.unwrap());
                }
                Err(e) => eprintln!("Error processing game: {:?}", e),
            }
        })
        .await;
    Ok(ChessDataResponse { time: 15 })
}
