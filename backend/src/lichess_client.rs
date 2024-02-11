use crate::api::{ChessDataRequest, ChessDataResponse, GameFetchWarning};
use crate::data_processor::{get_half_time_differentials, process_average_time, process_win_rate};
use crate::deserialization::GameJson;
use crate::game_info_generator::{generate_game_info_struct, GameInfo};
use crate::message_generator::{
    get_average_time_string_fmt, get_explanation_message, get_win_ratio_string_fmt,
};
use crate::trend_chart_generator::process_trend_chart_data;
use crate::util::generate_dummy_erros_testing;

use actix_web::{Error, HttpResponse};
use futures_util::{StreamExt, TryStreamExt};
use reqwest::Response;
use serde_json::{self};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn get_url(request_data: &ChessDataRequest) -> String {
    format!(
        "https://lichess.org/api/games/user/{}?max={}&perfType={}&color={}&rated=true&clocks=true",
        request_data.username,
        request_data.games_count,
        request_data.game_mode,
        request_data.user_color
    )
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

pub async fn process_response(
    games_info: &mut Vec<GameInfo>,
    request_data: ChessDataRequest,
    request_response: Response,
    skipped_games: &mut HashMap<usize, GameFetchWarning>,
) -> Result<(), Box<dyn std::error::Error>> {
    let games_info_arc = Arc::new(Mutex::new(games_info));
    let skipped_games_arc = Arc::new(Mutex::new(skipped_games));
    let mut game_idx = 0;

    let stream = request_response.bytes_stream();
    let username = request_data.username;
    stream
        .try_for_each_concurrent(None, move |game_bytes| {
            let games_info_ref = games_info_arc.clone();
            let skipped_games_ref = skipped_games_arc.clone();
            let username_ref = username.clone();

            async move {
                match serde_json::from_slice::<GameJson>(&game_bytes) {
                    Ok(game_json) => {
                        let mut lock = games_info_ref.lock().await;
                        lock.push(generate_game_info_struct(
                            &game_json,
                            &game_idx,
                            &username_ref,
                        ));
                    }
                    Err(e) => {
                        let mut lock = skipped_games_ref.lock().await;
                        lock.entry(game_idx).or_insert_with(|| {
                            GameFetchWarning::InternalErrorOccuredWhileProcessingAGame
                        });
                    }
                }
                game_idx += 1;
                Ok(())
            }
        })
        .await?;

    Ok(())
}

pub async fn handle_successful_response(
    request_data: ChessDataRequest,
    response: Response,
) -> HttpResponse {
    let mut skipped_games: HashMap<usize, GameFetchWarning> = HashMap::new();

    let mut games_info: Vec<GameInfo> = Vec::new();
    match process_response(&mut games_info, request_data, response, &mut skipped_games).await {
        Ok(_) => {
            let half_time_differentials =
                get_half_time_differentials(&games_info, &mut skipped_games, false);
            let average_time = process_average_time(&half_time_differentials);
            let win_rate = process_win_rate(&games_info, &skipped_games);
            let trend_chart_data =
                process_trend_chart_data(&games_info, &skipped_games, half_time_differentials);

            // For UI testing purposes:
            //    Adding a bunch of games with error message for errors side panel
            generate_dummy_erros_testing(&mut skipped_games);

            HttpResponse::Ok().json(ChessDataResponse::new(
                get_average_time_string_fmt(average_time),
                get_explanation_message(average_time),
                skipped_games,
                get_win_ratio_string_fmt(win_rate),
                trend_chart_data,
            ))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub fn handle_unsuccessful_response() -> HttpResponse {
    HttpResponse::InternalServerError().body("Lichess API returned non-success status")
}

pub async fn fetch_lichess_player_data(request_data: ChessDataRequest) -> HttpResponse {
    let url = get_url(&request_data);
    let client = reqwest::Client::new();

    match client
        .get(&url)
        .header("Accept", "application/x-ndjson")
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                handle_successful_response(request_data, response).await
            } else {
                handle_unsuccessful_response()
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch data from Lichess API"),
    }
}
