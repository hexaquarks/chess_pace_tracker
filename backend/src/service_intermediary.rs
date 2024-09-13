use std::collections::HashMap;
use std::time::Instant;

use crate::database;
use crate::deserialization;
use crate::lichess_client;
use crate::trend_chart_generator::TrendChartDatum;
use crate::websocket;
use crate::websocket::StopWebsocket;
use crate::websocket::WebSocketSession;

use actix_web::ResponseError;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use actix::Addr;
use websocket::AppState;
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
    pub user_elo: Option<i32>, // For internal uses only
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ChessDataResponse {
    RequestFromFrontend {
        time: String,
        explanation_message: (String, DescriptionMessageAssessment),
        games_with_errors: Vec<(usize, String)>,
        trend_chart_data: Vec<TrendChartDatum>,
        player_win_rate_in_fetched_games: String,
        players_flag_counts: (i32, i32),
    },
    RequestFromDatabase {
        time: String,
        players_considered: Vec<(String, i32)>,
    },
}

impl ChessDataResponse {
    pub fn new(
        time: String,
        explanation_message: (String, DescriptionMessageAssessment),
        games_with_errors: HashMap<usize, GameFetchWarning>,
        trend_chart_data: Vec<TrendChartDatum>,
        player_win_rate_in_fetched_games: String,
        players_flag_counts: (i32, i32),
    ) -> Self {
        let errors_vec =
        deserialization::convert_games_with_errors_to_displayable_format(games_with_errors);

        ChessDataResponse::RequestFromFrontend {
            time,
            explanation_message,
            games_with_errors: errors_vec,
            trend_chart_data,
            player_win_rate_in_fetched_games,
            players_flag_counts,
        }
    }

    pub fn new_internal(time: String, players_considered: Vec<(String, i32)>) -> Self {
        ChessDataResponse::RequestFromDatabase {
            time,
            players_considered,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RequestSource {
    Frontend,
    Internal,
}

impl RequestSource {
    fn from_str(requested_by: Option<&str>) -> Self {
        match requested_by {
            Some("frontend") => RequestSource::Frontend,
            Some("internal") => RequestSource::Internal,
            _ => {
                assert!(
                    true,
                    "Request source not recognized. Defaulting to 'frontend'"
                );
                RequestSource::Frontend
            }
        }
    }
}

pub fn get_websocket_address(
    requested_by: &RequestSource,
    app_state: &web::Data<AppState>,
) -> Option<Addr<WebSocketSession>> {
    match requested_by {
        RequestSource::Frontend => {
            let websocket_session = app_state.websocket_session.lock().unwrap();
            websocket_session.clone()
        }
        _ => None,
    }
}

pub async fn close_websocket(
    opt_websocket_addr: &Option<Addr<WebSocketSession>>, 
    app_state: &web::Data<AppState>,
) -> Result<(), std::string::String> {
    if let Some(websocket_addr) = opt_websocket_addr {
        websocket_addr
            .send(websocket::WebSocketTextMessage("All games processed. Closing connection.".to_string()))
            .await
            .map_err(|e| format!("The websocket was not properly closed: {:?}", e))?;
        websocket_addr
            .send(StopWebsocket)
            .await
            .map_err(|e| format!("The websocket was not properly closed: {:?}", e))?;

        // Remove the WebSocket session from AppState. It will be reinstanciated on the next POST
        // request.
        let mut websocket_session = app_state.websocket_session.lock().unwrap();
        *websocket_session = None;
    }

    Ok(())
}

#[post("/fetch-chess-data")]
pub async fn fetch_chess_data(
    info: web::Json<ChessDataRequest>,
    req: HttpRequest,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let start_time = Instant::now();
    let requested_by = RequestSource::from_str(
        req.headers()
            .get("x-requested-by")
            .and_then(|header_value| header_value.to_str().ok()),
    );

    // Channel game processing updates to the client through a websocket.
    let opt_websocket_addr = get_websocket_address(&requested_by, &app_state);

    // Fetch player data and send updates via WebSocket for accurate progression rate.
    let fetch_result = lichess_client::fetch_player_data(&info, requested_by, &opt_websocket_addr).await;

    // TODO: Handle error. Close the WebSocket after processing all games.
    close_websocket(&opt_websocket_addr, &app_state).await.ok();
        
    match fetch_result {
        Ok(response) => {
            let end_time = Instant::now();
            let processing_time = end_time.duration_since(start_time).as_secs_f32();

            match database::log_request_data(
                info.games_count,
                info.game_mode.as_str(),
                info.user_color.as_str(),
                info.user_elo,
                processing_time,
            ) {
                Ok(_) => response,
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(e) => e.error_response(),
    }
}
