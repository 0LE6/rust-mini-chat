use actix::prelude::*;
use actix_web_actors::ws;
use std::collections::HashSet;
use super::models::MyWebSocketMessage;
use actix_web::{web, Error, HttpRequest, HttpResponse};
//use bytestring::ByteString;

pub struct WebSocket {
    sessions: HashSet<Addr<WebSocket>>,
}

impl WebSocket {
    pub fn new() -> Self {
        WebSocket {
            sessions: HashSet::new(),
        }
    }
}

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.sessions.insert(ctx.address());
        println!("WebSocket session started");
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        self.sessions.remove(&ctx.address());
        println!("WebSocket session stopped");
        Running::Stop
    }
}

impl Handler<MyWebSocketMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: MyWebSocketMessage, ctx: &mut Self::Context) -> Self::Result {
        for addr in &self.sessions {
            if *addr != ctx.address() {
                let _ = addr.do_send(MyWebSocketMessage(msg.0.clone()));
            }
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("Message received: {:?}", text);
                let message = MyWebSocketMessage(text.to_string()); // Convert ByteString to String
                for session in &self.sessions {
                    if *session != ctx.address() {
                        let _ = session.do_send(message.clone()); // Send MyWebSocketMessage
                    }
                }
            },
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg)
            },
            Ok(ws::Message::Binary(bin)) => {
                ctx.binary(bin)
            },
            Err(e) => println!("Error: {:?}", e),
            _ => (),
        }
    }
}


// Make sure this is public and correctly typed
pub async fn ruta_del_chat(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WebSocket::new(), &req, stream)
}
