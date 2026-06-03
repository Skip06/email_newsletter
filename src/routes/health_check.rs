use actix_web::{ HttpResponse};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish() //OK() gives httpresponseBuilder but we need response so to return response with empty body => .finish() but ok() also implements Responder so works both ways
}
