use actix::Addr;
use std::collections::HashMap;

use crate::deserialization::GameJson;
use crate::games_info_generator::{self, get_opponents_and_their_rating, GameInfo};
use crate::games_info_processor::{
    get_half_time_differentials, process_average_time, process_flag_info, process_win_rate,
};
use crate::insight_generator::{self, InsightsPanelProps};
use crate::service_intermediary::{
    ChessDataRequest, ChessDataResponse, GameFetchWarning, RequestSource,
};
use crate::trend_chart_generator;
use crate::util;
use crate::websocket::WebSocketSession;

use actix_web::{Error, HttpResponse};
use futures_util::TryStreamExt;
use reqwest::Response;
use serde_json;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::Mutex;
use tokio_util::io::StreamReader;

fn convert_err(err: reqwest::Error) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
}

pub fn get_url(request_data: &ChessDataRequest) -> String {
    // Note the color query parameter acts like a filter. If the user_color in the
    // request structure contains "both", we omit the color query parameter all together.
    format!(
        "https://lichess.org/api/games/user/{}?max={}&perfType={}{}&rated=true&clocks=true",
        request_data.username,
        request_data.games_count,
        request_data.game_mode,
        if request_data.user_color == "both" {
            String::new()
        } else {
            format!("&color={}", request_data.user_color)
        }
    )
}

pub async fn process_response_stream(
    games_info: &mut Vec<GameInfo>,
    request_data: &ChessDataRequest,
    request_response: Response,
    skipped_games: &mut HashMap<usize, GameFetchWarning>,
    opt_websocket_addr: &Option<Addr<WebSocketSession>>,
) -> Result<(), Error> {
    let games_info_arc = Arc::new(Mutex::new(games_info));
    let skipped_games_arc = Arc::new(Mutex::new(skipped_games));
    let opt_websocket_addr_arc = Arc::new(Mutex::new(opt_websocket_addr.clone()));
    let game_idx_arc = Arc::new(Mutex::new(0));

    let stream = request_response.bytes_stream().map_err(convert_err);
    let stream_reader = StreamReader::new(stream);
    let mut lines = BufReader::new(stream_reader).lines();

    while let Some(line) = lines.next_line().await? {
        let mut game_idx_lock = game_idx_arc.lock().await;
        let username = request_data.username.clone();

        match serde_json::from_str::<GameJson>(&line) {
            Ok(game_json) => {
                let mut games_info_lock = games_info_arc.lock().await;
                games_info_lock.push(games_info_generator::generate(
                    &game_json,
                    &game_idx_lock,
                    &username,
                ));

                // Notify client that one of the games requested has been processed (for loading bar).
                if opt_websocket_addr.is_some() {
                    let websocket_addr_lock = opt_websocket_addr_arc.lock().await;
                    util::send_websocket_message(
                        &websocket_addr_lock.clone().unwrap(),
                        game_idx_lock.clone(),
                        &request_data.games_count,
                    );
                }

                *game_idx_lock += 1;
            }
            Err(_) => {
                let mut skipped_games_lock = skipped_games_arc.lock().await;
                skipped_games_lock
                    .entry(*game_idx_lock)
                    .or_insert_with(|| GameFetchWarning::InternalErrorOccuredWhileProcessingAGame);
                *game_idx_lock += 1;
            }
        }
    }

    Ok(())
}

// TODO add specializations for this function
pub async fn handle_successful_response(
    request_data: &ChessDataRequest,
    requested_by: RequestSource,
    response: Response,
    opt_websocket_addr: &Option<Addr<WebSocketSession>>,
) -> Result<HttpResponse, Error> {
    let mut skipped_games: HashMap<usize, GameFetchWarning> = HashMap::new();
    let mut games_info: Vec<GameInfo> = Vec::new();

    // =========== STEP 1: Process the response stream ===========
    process_response_stream(
        &mut games_info,
        request_data,
        response,
        &mut skipped_games,
        opt_websocket_addr,
    )
    .await?;

    // =========== STEP 2: Get the half time differentials ===========
    let half_time_differentials: Vec<f32> =
        get_half_time_differentials(&games_info, &mut skipped_games, false);

    // =========== STEP 3: Get average time ===========
    // Note: Average time might be None if 0 games were kept for the computation.
    let average_time = process_average_time(&half_time_differentials);

    // If the request was made internally for statistics, we only need to return the average time.
    if requested_by == RequestSource::Internal {
        match average_time {
            Some(time) => {
                return Ok(HttpResponse::Ok().json(ChessDataResponse::new_internal(
                    time.to_string(),
                    get_opponents_and_their_rating(&games_info),
                )));
            }
            None => {
                return Err(actix_web::error::ErrorInternalServerError(
                    "Data processing was incomplete for the requested sample.",
                ));
            }
        }
    }

    // =========== STEP 4: Process win rate ===========
    let win_rate = process_win_rate(&games_info, &skipped_games);

    // =========== STEP 5: Process flag info ===========
    let (user_flag_count, opponent_flag_cout) = process_flag_info(&games_info, &skipped_games);

    // =========== STEP 6: Generate Trend Chart Data ===========
    let trend_chart_data =
        trend_chart_generator::generate(&games_info, &skipped_games, half_time_differentials);

    // =========== STEP 7: Generate Insights ===========
    let insights: InsightsPanelProps =
        insight_generator::get_insights(average_time, win_rate, request_data);

    // For UI testing purposes:
    //    Adding a bunch of games with error message for errors side panel
    // util::generate_dummy_erros_testing(&mut skipped_games);

    Ok(HttpResponse::Ok().json(ChessDataResponse::new(
        insights.average_time,
        insights.explanation_message,
        skipped_games,
        trend_chart_data,
        insights.win_ratio,
        (user_flag_count, opponent_flag_cout),
    )))
}

pub async fn fetch_player_data(
    request_data: &ChessDataRequest,
    requested_by: RequestSource,
    opt_websocket_addr: &Option<Addr<WebSocketSession>>,
) -> Result<HttpResponse, Error> {
    let url = get_url(&request_data);
    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .header("Accept", "application/x-ndjson")
        .send()
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Request error: {:?}", e))
        })?;

    if response.status().is_success() {
        handle_successful_response(request_data, requested_by, response, opt_websocket_addr).await
    } else {
        let status = response.status();
        let error_message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Ok(HttpResponse::build(status).json(serde_json::json!({ "error": error_message })))
    }
}
