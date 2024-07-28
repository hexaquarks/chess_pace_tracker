mod database;
mod deserialization;
mod errors_manager;
mod flagging_info_generator;
mod games_info_generator;
mod games_info_processor;
mod insight_generator;
mod lichess_client;
mod service_intermediary;
mod trend_chart_generator;
mod unit_test_util;
mod util;
mod websocket;

use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, HttpServer};

const BIND_ADDRESS: &str = "127.0.0.1:8000";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    database::create_database()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
                // We add x-requested-by custom header to distinguish between clien requests
                // and internal requests.
                header::HeaderName::from_static("x-requested-by"),
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(service_intermediary::fetch_chess_data)
            .service(web::resource("/ws").route(web::get().to(websocket::add_websocket_endpoint)))
            .wrap(middleware::Logger::default())
    })
    .bind(BIND_ADDRESS)?
    .run()
    .await
}
