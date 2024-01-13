use crate::fetch_lichess::fetch_lichess_player_data;
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub enum DescriptionMessageAssessment {
    Positive,
    Neutral,
    Negative,
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
    pub time: f32,
    pub explanation_message: (String, DescriptionMessageAssessment),
}

#[post("/fetch-chess-data")]
pub async fn fetch_chess_data(info: web::Json<ChessDataRequest>) -> impl Responder {
    match fetch_lichess_player_data(info.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
