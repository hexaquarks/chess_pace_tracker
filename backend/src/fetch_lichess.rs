use crate::api::{ChessDataRequest, ChessDataResponse};
use crate::deserialization::GameJson;
use actix_web::Error;
use futures_util::StreamExt;
use reqwest::Response;
use serde_json::{self};

pub fn get_url(request_data: ChessDataRequest) -> String {
    let url = format!(
        "https://lichess.org/api/games/user/{}?max={}&rated=true&clocks=true",
        request_data.username, request_data.games_count,
    );
    url
}

pub async fn send_request(request_data: ChessDataRequest) -> Result<Response, Error> {
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

pub fn generate_timed_moves(game: &GameJson) -> Vec<(&str, i64)> {
    let mut timed_moves: Vec<(&str, i64)> = Vec::new();

    let moves: Vec<&str> = game.moves.as_ref().unwrap().split(" ").collect::<Vec<_>>();
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

    for (i, &x) in moves.iter().enumerate() {
        timed_moves.push((x, clocks[i]));
    }
    timed_moves
}

pub async fn fetch_lichess_player_data(
    request_data: ChessDataRequest,
) -> Result<ChessDataResponse, Error> {
    let request_response = send_request(request_data).await?;

    if !request_response.status().is_success() {
        return Err(Error::from(actix_web::error::ErrorInternalServerError(
            "Lichess API returned non-success status",
        )));
    }

    let mut games_info: Vec<GameJson> = Vec::new();
    let stream = request_response.bytes_stream();
    stream
        .for_each(|game_result| {
            match game_result {
                Ok(game_bytes) => {
                    let game: GameJson =
                        serde_json::from_slice(&game_bytes).expect("Failed to serialize GameJson");

                    games_info.push(game);
                }
                Err(e) => eprintln!("Error processing game: {:?}", e),
            }
            async { () } // <-- Jfl absolutely cursed
        })
        .await;

    let timed_moves: Vec<Vec<(&str, i64)>> = games_info
        .iter()
        .map(|game| generate_timed_moves(&game))
        .collect::<Vec<_>>();

    Ok(ChessDataResponse { time: 15 })
}
