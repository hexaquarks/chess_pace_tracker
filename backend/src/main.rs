mod deserialization;
mod games_info_generator;
mod games_info_processor;
mod insight_generator;
mod lichess_client;
mod service_intermediary;
mod trend_chart_generator;
mod unit_test_util;
mod util;

use actix_cors::Cors;
use actix_web::{http::header, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(service_intermediary::fetch_chess_data)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
