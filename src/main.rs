use actix_web::{web, App, HttpServer, Responder};
use listenfd::ListenFd;
use std::io;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new());

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3005")?
    };

    server.run().await
}
