use actix_web::dev::Server;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use std::net::TcpListener;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish() //OK() gives httpresponseBuilder but we need response so to return response with empty body => .finish() but ok() also implements Responder so works both ways
}
