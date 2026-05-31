
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};

async fn greet (req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", name)
    
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()  //OK() gives httpresponseBuilder but we need response so to return response with empty body => .finish() but ok() also implements Responder so works both ways
}


#[tokio::main]
async fn main()-> Result<(), std::io::Error> {
    HttpServer::new( || {
        App::new()
            
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(greet))
    })
    .bind("localhost:8000")? //bind returns result 
    .run()
    .await
}
#[cfg(test)]
mod tests{

    use super::*;

    #[tokio::test]
    async fn health_check_success() {
        let response = health_check().await;

        assert!(response.status().is_success()) // had to change return type of health_check() 
    }
}
