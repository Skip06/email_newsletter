use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};

async fn greet (req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", name)
    
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()  //OK() gives httpresponseBuilder but we need response so to return response with empty body => .finish() but ok() also implements Responder so works both ways
}


// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation
pub async fn run()-> Result<(), std::io::Error> {
    HttpServer::new( || {
        App::new()
            
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(greet))
    })
    .bind("localhost:8000")? //bind returns result 
    .run()
    .await
}