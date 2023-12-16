use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ChessDataRequest {
    username: String,
    games_count: String,
}

#[derive(Serialize)]
struct ChessDataResponse {
    time: i32,
}

#[post("/fetch-chess-data")]
async fn fetch_chess_data(info: web::Json<ChessDataRequest>) -> impl Responder {
    let response = ChessDataResponse { time: 13 };

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new().wrap(cors).service(fetch_chess_data)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
