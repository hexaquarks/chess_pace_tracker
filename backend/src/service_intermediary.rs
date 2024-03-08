use std::collections::HashMap;
use std::time::Instant;

use crate::database;
use crate::deserialization;
use crate::lichess_client;
use crate::trend_chart_generator::TrendChartDatum;

use actix_web::ResponseError;
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(i32)]
pub enum DescriptionMessageAssessment {
    Positive = 0,
    Neutral = 1,
    Negative = 2,
}

#[derive(Serialize, Debug, PartialEq, Eq, Hash)]
pub enum GameFetchWarning {
    InternalErrorOccuredWhileProcessingAGame = 0,
    GameHasNotEnoughMoves,
}

#[derive(Serialize)]
pub enum GlobalFetchError {
    RequestedMoreGamesThanAvailableInTheUserDatabase,
    NotEnoughGamesToComputeAverage, // n == 0
}

#[derive(Deserialize, Debug)]
pub struct ChessDataRequest {
    pub username: String,
    pub games_count: i32,
    pub game_mode: String,
    pub user_color: String,
}

#[derive(Serialize)]
pub struct ChessDataResponse {
    pub time: String,
    pub explanation_message: (String, DescriptionMessageAssessment),
    pub games_with_errors: Vec<(usize, String)>,
    pub player_win_rate_in_fetched_games: String,
    pub trend_chart_data: Vec<TrendChartDatum>,
    pub players_flag_counts: (i32, i32),
}

impl ChessDataResponse {
    pub fn new(
        time: String,
        explanation_message: (String, DescriptionMessageAssessment),
        games_with_errors: HashMap<usize, GameFetchWarning>,
        player_win_rate_in_fetched_games: String,
        trend_chart_data: Vec<TrendChartDatum>,
        players_flag_counts: (i32, i32),
    ) -> Self {
        let errors_vec =
            deserialization::convert_games_with_errors_to_displayable_format(games_with_errors);

        ChessDataResponse {
            time,
            explanation_message,
            games_with_errors: errors_vec,
            player_win_rate_in_fetched_games,
            trend_chart_data,
            players_flag_counts,
        }
    }
}

#[post("/fetch-chess-data")]
pub async fn fetch_chess_data(info: web::Json<ChessDataRequest>) -> impl Responder {
    let start_time = Instant::now();

    match lichess_client::fetch_player_data(&info).await {
        Ok(response) => {
            let end_time = Instant::now();
            let processing_time = end_time.duration_since(start_time).as_secs_f32();

            match database::log_request_data(
                info.games_count,
                info.game_mode.as_str(),
                info.user_color.as_str(),
                processing_time,
            ) {
                Ok(_) => response,
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(e) => e.error_response(),
    }
}
