
use actix_web::{App, web, HttpRequest,HttpServer, Responder};

async fn greet (req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", name)
    
}


#[tokio::main]
async fn main()-> Result<(), std::io::Error> {
    HttpServer::new( || {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("localhost:8000")? //bind returns result 
    .run()
    .await
}

/*
HttpServer, in other words, handles all transport level concerns.
What happens afterwards? What does HttpServer do when it has established a new connection with a client
of our API and we need to start handling their requests?
That is where App comes into play!
*/