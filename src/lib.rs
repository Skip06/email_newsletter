use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;
use std::net::TcpListener;



async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()  //OK() gives httpresponseBuilder but we need response so to return response with empty body => .finish() but ok() also implements Responder so works both ways
}
#[derive(serde::Deserialize)]
struct FormData{
    name: String,
    email: String
}


//Handlers
// this form acts as middleware where it checks all params in the FormData are there or not otherswise the 2nd test case pass with 400
async fn subscribe(form: web::Form<FormData>) -> HttpResponse{ //currently it returns 200 for every req but the 2nd testcase requires it to pass with 400.
    println!("HANDLER CALLED");
    HttpResponse::Ok().finish()
}


//if u are not awaiting then remove the async also otherwise this will return Future<Result<T,E>>
pub fn run(listener : TcpListener)-> Result<Server, std::io::Error> { // now it returns a Server type which out main fn can await and use it as it was and test can run it as a background task using tokio::spawn()
    let server = HttpServer::new( || {
        App::new()
            
            .route("/health_check", web::get().to(health_check))
            .route("/subscription", web::post().to(subscribe))
    })
    .listen(listener)? //bind returns result 
    .run()   ;          // .run() returns a Server type which you can think of a Future which will run by .await
    Ok(server)
}