use crate::api::{ChessDataRequest, ChessDataResponse};
use crate::data_processor::process_average_time;
use crate::deserialization::GameJson;
use actix_web::Error;
use futures_util::StreamExt;
use reqwest::Response;
use serde_json::{self};
use std::collections::HashMap;

pub fn get_url(request_data: &ChessDataRequest) -> String {
    let url = format!(
        "https://lichess.org/api/games/user/{}?max={}&perfType={}&color={}&rated=true&clocks=true",
        request_data.username,
        request_data.games_count,
        request_data.game_mode,
        request_data.user_color
    );
    url
}

pub async fn send_request(request_data: &ChessDataRequest) -> Result<Response, Error> {
    let client = reqwest::Client::new();

    let res = client
        .get(get_url(request_data))
        .header("Accept", "application/x-ndjson")
        .send()
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Request failed: {:?}", e))
        });
    res
}

#[derive(Clone)]
pub struct TimedMove {
    pub move_key: String,
    pub move_time: i64,
}

pub fn generate_timed_moves(game: &GameJson) -> Vec<TimedMove> {
    let mut timed_moves: Vec<TimedMove> = Vec::new();

    let moves: Vec<String> = game
        .moves
        .as_ref()
        .unwrap()
        .split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let mut clocks: Vec<i64> = game
        .clocks
        .as_ref()
        .unwrap()
        .iter()
        .cloned()
        .collect::<Vec<_>>();

    if clocks.len() > moves.len() {
        // if the last move in the game was a checkmate, the last
        // clock stamp does not register.
        clocks.truncate(clocks.len() - 1);
    }

    for (i, x) in moves.iter().cloned().enumerate() {
        timed_moves.push(TimedMove {
            move_key: x,
            move_time: clocks[i],
        });
    }
    timed_moves
}

// UI -> Panel with scroll bar errors
//    -> maybe warning and error sections?
// for instance, game 5 -> not enough moves,

// Skip games in fetch
pub enum GameFetchWarning {
    GameHasNotEnoughMoves,
    InternalErrorOccuredWhileProcessingAGame,
}

pub enum GlobalFetchError {
    RequestedMoreGamesThanAvailableInTheUserDatabase,
    NotEnoughGamesToComputeAverage, // n == 0
}

pub struct GameInfo {
    pub game_index: usize,
    pub timed_moves: Vec<TimedMove>,
    pub user_color: String,
    pub user_rating: i32,
    pub opponent_rating: i32,
    pub winner_color: String,
}

pub fn get_user_color(game: &GameJson, user_name: &str) -> String {
    if let Some(players) = game.players.as_ref() {
        if let Some(black_player) = players.black.as_ref() {
            if let Some(user) = black_player.user.as_ref() {
                if user.name.as_ref() == Some(&user_name.to_string()) {
                    return "black".to_string();
                }
            }
        }
    }
    "white".to_string()
}

pub fn get_user_rating(game: &GameJson, user_color: &str) -> i32 {
    let players = match game.players.as_ref() {
        Some(players) => players,
        None => return 0,
    };

    let player_detail = match user_color {
        "black" => &players.black,
        _ => &players.white,
    };

    player_detail.as_ref().unwrap().rating.unwrap_or(0)
}

pub fn get_winner_color(game: &GameJson) -> String {
    game.winner.as_ref().unwrap().clone()
}

pub fn generate_game_info_struct(
    game: &GameJson,
    game_idx: &usize,
    user_name: &String,
) -> GameInfo {
    let user_color = get_user_color(game, user_name);
    let user_rating = get_user_rating(game, &user_color);
    let opponent_color = if user_color == "black" {
        "white"
    } else {
        "black"
    };
    let opponent_rating = get_user_rating(game, opponent_color);
    let game_info = GameInfo {
        game_index: *game_idx,
        timed_moves: generate_timed_moves(game),
        user_color: user_color,
        user_rating: user_rating,
        opponent_rating: opponent_rating,
        winner_color: get_winner_color(game),
    };
    game_info
}

pub async fn fetch_lichess_player_data(
    request_data: ChessDataRequest,
) -> Result<ChessDataResponse, Error> {
    let request_response = send_request(&request_data).await?;

    if !request_response.status().is_success() {
        return Err(Error::from(actix_web::error::ErrorInternalServerError(
            "Lichess API returned non-success status",
        )));
    }

    let mut skipped_games: HashMap<usize, GameFetchWarning> = HashMap::new();
    let mut games_info: Vec<GameInfo> = Vec::new();
    let mut game_idx: usize = 0;
    let stream = request_response.bytes_stream();
    stream
        .for_each(|game_result| {
            match game_result {
                Ok(game_bytes) => {
                    let game: GameJson =
                        serde_json::from_slice(&game_bytes).expect("Failed to serialize GameJson");

                    let game_info =
                        generate_game_info_struct(&game, &game_idx, &request_data.username);
                    games_info.push(game_info);
                    game_idx += 1;
                }
                Err(e) => {
                    eprintln!("Error processing game: {:?}", e);
                    skipped_games
                        .entry(game_idx)
                        .or_insert(GameFetchWarning::InternalErrorOccuredWhileProcessingAGame);
                    game_idx += 1;
                }
            }
            async { () } // <-- Jfl absolutely cursed
        })
        .await;

    let average_half_time_differential = process_average_time(&games_info, &mut skipped_games);

    Ok(ChessDataResponse {
        time: average_half_time_differential,
    })
}
