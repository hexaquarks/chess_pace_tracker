use actix::prelude::*;
use actix::{Actor, Handler, Message};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::time::{Duration, Instant};

use lazy_static::lazy_static;
use std::sync::Mutex;
use uuid::Uuid;

// Struct to store the WebSocket session and share across handlers
pub struct AppState {
    pub websocket_session: Mutex<Option<Addr<WebSocketSession>>>,
}

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct WebSocketTextMessage(pub String);

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct StopWebsocket;

pub struct WebSocketSession {
    heart_beat: Instant,
    session_id: String,
    app_state: web::Data<AppState>,
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heart_beat(ctx);
    }
}

impl Handler<WebSocketTextMessage> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: WebSocketTextMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl Handler<StopWebsocket> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, _: StopWebsocket, ctx: &mut Self::Context) {
        ctx.stop();
    }
}
impl WebSocketSession {
    pub fn new(session_id: String, app_state: web::Data<AppState>) -> Self {
        WebSocketSession {
            heart_beat: Instant::now(),
            session_id,
            app_state,
        }
    }

    fn heart_beat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.heart_beat) > CLIENT_TIMEOUT {
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }

    fn stop_gracefully(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        // Shouldn't be getting in here for now. 
        println!("Websocket stopped gracefully.");

        // Remove the address from the app state when the session is stopped
        let mut websocket_session = self.app_state.websocket_session.lock().unwrap();
        *websocket_session = None;

        ctx.stop();
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.heart_beat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.heart_beat = Instant::now();
            }
            Ok(ws::Message::Close(_)) => {
                // In case in the future I add functionality from the client side to manually
                // close the server websocket.
                self.stop_gracefully(ctx);
            }
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => ctx.stop(),
        }
    }
}

pub async fn add_websocket_endpoint(
    request: HttpRequest,
    stream: web::Payload,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // Generate a unique session ID for the new WebSocket connection
    let session_id = Uuid::new_v4().to_string();
    let ws_session = WebSocketSession::new(session_id.clone(), app_state.clone());

    // Start the WebSocket session and get its address
    let (addr, response) = ws::WsResponseBuilder::new(ws_session, &request, stream)
        .start_with_addr()?;

    // Store the WebSocket address in the app state. It will be handled in the POST request.
    {
        let mut websocket_session = app_state.websocket_session.lock().unwrap();
        *websocket_session = Some(addr.clone());
    }

    Ok(response)
}
