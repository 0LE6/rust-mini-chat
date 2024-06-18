mod websocket;
mod models;

use actix_web::{web, App, HttpServer};
use actix_files as fs;

// Esto fue una primera prueba de conexión con el servidor
// async fn prueba() -> impl Responder {
//     "Esto es una prueba de servidor Actix"
// }

#[actix_web::main] // atributo para iniciar el sistema Actix
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/", "./static").index_file("index.html"))
            .route("/ws", web::get().to(websocket::ruta_del_chat))
    })
        // .bind("127.0.0.1:5000")?
        .bind("0.0.0.0:5000")?
        .run()
        .await
}
