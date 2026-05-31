use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;



async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()  //OK() gives httpresponseBuilder but we need response so to return response with empty body => .finish() but ok() also implements Responder so works both ways
}



//if u are not awaiting then remove the async also otherwise this will return Future<Result<T,E>>
pub fn run()-> Result<Server, std::io::Error> { // now it returns a Server type which out main fn can await and use it as it was and test can run it as a background task using tokio::spawn()
    let server = HttpServer::new( || {
        App::new()
            
            .route("/health_check", web::get().to(health_check))
           
    })
    .bind("localhost:8000")? //bind returns result 
    .run()   ;          // .run() returns a Server type which you can think of a Future which will run by .await
    Ok(server)
}