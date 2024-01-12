use actix_cors::Cors;
use actix_web::{http::header, App, HttpServer};

mod api;
mod data_processor;
mod deserialization;
mod fetch_lichess;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new().wrap(cors).service(api::fetch_chess_data)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
