use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{App, HttpServer,  web};
use sqlx::PgConnection;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    
    // now it returns a Server type which out main fn can await and use it as it was and test can run it as a background task using tokio::spawn()
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {   //requires pgConnection to be cloneable READ ARCTIXWEb WORKERS 3.9.2 but it cant be so Arc smart pointer in web::Data()
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscription", web::post().to(subscribe))
            .app_data(connection.clone())    //AppState which is shared by whole app
    })
    
    .listen(listener)? 
    .run(); // .run() returns a Server type which you can think of a Future which will run by .await
    Ok(server)
}

/*
actix-web demands a separate copy of the database connection for each of its workers.
Rust says: "Absolutely not, you cannot copy a physical network connection."

Instead of trying to make copies of the network connection itself, we wrap our connection inside web::Data, which uses a special Rust container called an Arc (Atomic Reference Counted pointer).
Think of an Arc like a secured glass display case at a museum, and your network connection is the valuable artifact inside it.
    You cannot clone the artifact
    But you can hand out cheap, numbered digital entry tickets (pointers) to all the CPU workers.
*/