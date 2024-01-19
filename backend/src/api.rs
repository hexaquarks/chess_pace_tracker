use crate::fetch_lichess::fetch_lichess_player_data;
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(i32)]
pub enum DescriptionMessageAssessment {
    Positive = 0,
    Neutral = 1,
    Negative = 2,
}

#[derive(Serialize, Debug)]
pub enum GameFetchWarning {
    GameHasNotEnoughMoves,
    InternalErrorOccuredWhileProcessingAGame,
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
    pub games_with_errors: Vec<(usize, GameFetchWarning)>,
}

impl ChessDataResponse {
    pub fn new(
        time: String,
        explanation_message: (String, DescriptionMessageAssessment),
        games_with_errors: HashMap<usize, GameFetchWarning>,
    ) -> Self {
        println!("HashMap contents: {:?}", games_with_errors);
        let errors_vec = games_with_errors.into_iter().collect::<Vec<_>>();

        ChessDataResponse {
            time,
            explanation_message,
            games_with_errors: errors_vec,
        }
    }
}

#[post("/fetch-chess-data")]
pub async fn fetch_chess_data(info: web::Json<ChessDataRequest>) -> impl Responder {
    match fetch_lichess_player_data(info.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
