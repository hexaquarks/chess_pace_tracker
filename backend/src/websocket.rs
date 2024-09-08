use actix::prelude::*;
use actix::{Actor, Handler, Message};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::time::{Duration, Instant};

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref WEBSOCKET_ADDR: Mutex<Option<Addr<WebSocketSession>>> = Mutex::new(None);
}

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct WebSocketTextMessage(pub String);

pub struct WebSocketSession {
    heart_beat: Instant,
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

impl WebSocketSession {
    fn new() -> Self {
        Self {
            heart_beat: Instant::now(),
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
        println!("Stopping WebSocket session gracefully");

        // Remove the address from the global WEBSOCKET_ADDR
        let mut websocket_addr = WEBSOCKET_ADDR.lock().unwrap();
        *websocket_addr = None;

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
) -> Result<HttpResponse, Error> {
    println!("WebSocket endpoint hit");
    let ws_session = WebSocketSession::new();
    let (addr, response) =
    ws::WsResponseBuilder::new(ws_session, &request, stream).start_with_addr()?;

    // Store the WebSocket address in the global variable
    {
        println!("Storing WebSocket address");
        let mut websocket_addr = WEBSOCKET_ADDR.lock().unwrap();
        *websocket_addr = Some(addr);
    }

    Ok(response)
}
