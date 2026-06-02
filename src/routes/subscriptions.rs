use actix_web::dev::Server;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

//Handlers
// this form acts as middleware where it checks all params in the FormData are there or not otherswise the 2nd test case pass with 400
pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    //currently it returns 200 for every req but the 2nd testcase requires it to pass with 400.
    println!("HANDLER CALLED");
    HttpResponse::Ok().finish()
}
