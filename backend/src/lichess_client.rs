use std::collections::HashMap;
use std::sync::Arc;

use crate::deserialization::GameJson;
use crate::errors_manager::{self, ProcessError};
use crate::games_info_generator::{self, GameInfo};
use crate::games_info_processor::{
    get_half_time_differentials, process_average_time, process_win_rate,
};
use crate::insight_generator::{self, InsightsPanelProps};
use crate::service_intermediary::{ChessDataRequest, ChessDataResponse, GameFetchWarning};
use crate::trend_chart_generator;
use crate::util;

use actix_web::HttpResponse;
use futures_util::TryStreamExt;
use reqwest::Response;
use serde_json;
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

pub async fn process_response_stream(
    games_info: &mut Vec<GameInfo>,
    request_data: ChessDataRequest,
    request_response: Response,
    skipped_games: &mut HashMap<usize, GameFetchWarning>,
) -> Result<(), ProcessError> {
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
                        lock.push(games_info_generator::generate(
                            &game_json,
                            &game_idx,
                            &username_ref,
                        ));
                    }
                    Err(_) => {
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
        .await
        .map_err(|_| ProcessError::InternalError {
            message: "An error occurred while fetching player data.".into(),
        })?;

    Ok(())
}

pub async fn handle_successful_response(
    request_data: ChessDataRequest,
    response: Response,
) -> Result<HttpResponse, ProcessError> {
    let mut skipped_games: HashMap<usize, GameFetchWarning> = HashMap::new();
    let mut games_info: Vec<GameInfo> = Vec::new();

    process_response_stream(&mut games_info, request_data, response, &mut skipped_games).await?;

    let half_time_differentials =
        get_half_time_differentials(&games_info, &mut skipped_games, false);
    let average_time = process_average_time(&half_time_differentials);
    let win_rate = process_win_rate(&games_info, &skipped_games);
    let trend_chart_data =
        trend_chart_generator::generate(&games_info, &skipped_games, half_time_differentials);

    // For UI testing purposes:
    //    Adding a bunch of games with error message for errors side panel
    util::generate_dummy_erros_testing(&mut skipped_games);

    let insights: InsightsPanelProps = insight_generator::get_insights(average_time, win_rate);

    Ok(HttpResponse::Ok().json(ChessDataResponse::new(
        insights.average_time,
        insights.explanation_message,
        skipped_games,
        insights.win_ratio,
        trend_chart_data,
    )))
}

pub async fn fetch_player_data(
    request_data: ChessDataRequest,
) -> Result<HttpResponse, ProcessError> {
    let url = get_url(&request_data);
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("Accept", "application/x-ndjson")
        .send()
        .await
        .map_err(|_| ProcessError::FetchError {
            message: "Failed to fetch data from Lichess API. Check your internet connection."
                .into(),
        })?;

    if response.status().is_success() {
        handle_successful_response(request_data, response).await
    } else {
        Err(ProcessError::FetchError {
            message: "Lichess API returned non-success status".into(),
        })
    }
}
