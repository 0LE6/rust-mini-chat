use actix::{Actor};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Error};
use actix::prelude::*;
use actix_web_actors::ws;
//use tokio::stream;

// Esto fue una primera prueba de conexión con el servidor
// async fn prueba() -> impl Responder {
//     "Esto es una prueba de servidor Actix"
// }


struct WebSocket {
}

// En Actix, un actor es un objeto que encapsula el estado y el comportamiento. Los actores comunican entre sí exclusivamente por mensajes.
impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                println!("Ping recibido: {:?}", msg);
                ctx.pong(&msg)
            },
            Ok(ws::Message::Text(text)) => {
                println!("Mensaje de texto recibido: {:?}", text);
            },
            Ok(ws::Message::Binary(bin)) => {
                println!("Datos binarios recibidos");
                ctx.binary(bin)
            },
            Err(e) => println!("Error: {:?}", e),
            _ => (),
        }
    }
}



// Ruta para iniciar la conexión WebSocket
async fn ruta_del_chat(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WebSocket {}, &req, stream)
}


#[actix_web::main] // atributo para iniciar el sistema Actix
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/ws", web::get().to(ruta_del_chat))
    })
        .bind("127.0.0.1:5000")?
        .run()
        .await
}
