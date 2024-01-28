use crate::api::{ChessDataRequest, ChessDataResponse, GameFetchWarning};
use crate::data_processor::{process_average_time, process_win_rate};
use crate::deserialization::GameJson;
use crate::game_info_generator::{generate_game_info_struct, GameInfo};
use crate::message_generator::{
    get_average_time_string_fmt, get_explanation_message, get_win_ratio_string_fmt,
};
use actix_web::Error;
use futures_util::StreamExt;
use reqwest::Response;
use serde_json::{self};
use std::collections::HashMap;
use std::fmt::Debug;

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

pub async fn get_games_info_from_response_stream(
    request_data: ChessDataRequest,
    request_response: Response,
    skipped_games: &mut HashMap<usize, GameFetchWarning>,
) -> Vec<GameInfo> {
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

    games_info
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
    let games_info: Vec<GameInfo> =
        get_games_info_from_response_stream(request_data, request_response, &mut skipped_games)
            .await;

    let average_half_time_differential = process_average_time(&games_info, &mut skipped_games);
    let player_win_rate_in_fetched_games = process_win_rate(&games_info, &skipped_games);

    Ok(ChessDataResponse::new(
        get_average_time_string_fmt(average_half_time_differential),
        get_explanation_message(average_half_time_differential),
        skipped_games,
        get_win_ratio_string_fmt(player_win_rate_in_fetched_games),
    ))
}
