use crate::api::{ChessDataRequest, ChessDataResponse};

use futures::stream::StreamExt;
use lichess_api::client::LichessApi;
use lichess_api::model::games::export::by_user::{GetQuery, GetRequest};
use lichess_api::model::games::export::Base;

use config::{Config, File};
use serde_json;

pub async fn fetch_lichess_player_data(
    request_data: ChessDataRequest,
) -> Result<ChessDataResponse, actix_web::Error> {
    let mut settings = Config::default();
    settings
        .merge(File::with_name("keys"))
        .expect("Failed to read keys file");

    let api_key = settings
        .get::<String>("api_key")
        .expect("API key not found in keys file");

    let client = reqwest::Client::new();
    let api_token = Some(api_key); // Replace with actual token
    let api = LichessApi::new(&client, api_token);

    // Construct the GetQuery object
    let base = Base {
        moves: true,
        pgn_in_json: true,
        tags: true,
        clocks: true,
        evals: false,
        accuracy: false,
        opening: false,
        literate: false,
        players: None,
    };

    let query = GetQuery {
        base: base,
        since: None,
        until: None,
        max: request_data.games_count as u64,
        vs: None,
        rated: Some(true),
        perf_type: None,
        color: None,
        analysed: None,
        ongoing: None,
        finished: None,
        last_fen: None,
        sort: None,
    };

    let request = GetRequest::new(&request_data.username, query);

    match api.export_games_of_user(request).await {
        Ok(stream) => {
            stream
                .for_each(|game_result| async {
                    match game_result {
                        Ok(game) => {
                            println!("{:#?}", game);
                        }
                        Err(e) => eprintln!("Error processing game: {:?}", e),
                    }
                })
                .await;

            Ok(ChessDataResponse { time: 14 })
        }
        Err(e) => {
            eprintln!("Failed to fetch games: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to fetch games",
            ))
        }
    }
}
