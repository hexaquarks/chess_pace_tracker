use crate::api::{ChessDataRequest, ChessDataResponse};
use actix_web::{web, Error};
use futures_util::StreamExt;
use reqwest;
use serde_json::{self, Value};
use tokio::io::{AsyncBufReadExt, BufReader};

pub async fn fetch_lichess_player_data(
    request_data: ChessDataRequest,
) -> Result<ChessDataResponse, Error> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://lichess.org/api/games/user/{}?max={}&rated=true&clocks=true&pgnInJson=true",
        request_data.username, request_data.games_count,
    );

    let res = client
        .get(&url)
        .header("Accept", "application/x-ndjson")
        .send()
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Request failed: {:?}", e))
        })?;

    if !res.status().is_success() {
        return Err(Error::from(actix_web::error::ErrorInternalServerError(
            "Lichess API returned non-success status",
        )));
    }

    let body = res.bytes_stream();
    let reader = BufReader::new(tokio_util::io::StreamReader::new(body));
    let mut lines = reader.lines();

    let mut games = Vec::new();

    while let Some(line) = lines
        .next_line()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Line error: {:?}", e)))?
    {
        let line = line.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Line error: {:?}", e))
        })?;
        let game: Value = serde_json::from_str(&line).map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("JSON parse error: {:?}", e))
        })?;
        games.push(game);
    }

    for game in games.iter() {
        println!("{}", serde_json::to_string_pretty(&game).unwrap());
    }
    // match json_result {
    //     Ok(json) => {
    //         // If you just want to print the JSON to the console for debugging:
    //         // println!("Received JSON: {:?}", json);
    //         println!("{}", serde_json::to_string_pretty(&json).unwrap());
    //     }
    //     Err(e) => {
    //         println!("Failed to parse JSON: {:?}", e);
    //     }
    // }
    // let data = res.json::<ChessDataResponse>().await.map_err(|_| {
    //     Error::from(actix_web::error::ErrorInternalServerError(
    //         "Failed to parse response from Lichess",
    //     ))
    // })?;
    println!("Here 3");
    Ok(ChessDataResponse { time: 15 })
}
