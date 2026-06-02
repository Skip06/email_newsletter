use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use std::net::TcpListener;

//if u are not awaiting then remove the async also otherwise this will return Future<Result<T,E>>
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // now it returns a Server type which out main fn can await and use it as it was and test can run it as a background task using tokio::spawn()
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscription", web::post().to(subscribe))
    })
    .listen(listener)? //bind returns result
    .run(); // .run() returns a Server type which you can think of a Future which will run by .await
    Ok(server)
}
