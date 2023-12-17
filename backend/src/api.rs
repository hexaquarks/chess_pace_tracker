use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ChessDataRequest {
    username: String,
    games_count: i32,
}

#[derive(Serialize)]
pub struct ChessDataResponse {
    time: i32,
}

#[post("/fetch-chess-data")]
pub async fn fetch_chess_data(info: web::Json<ChessDataRequest>) -> impl Responder {
    let response: ChessDataResponse = ChessDataResponse { time: 13 };
    HttpResponse::Ok().json(response)
}
