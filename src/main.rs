use actixweb_email_newsletter::configuration::get_configuration;
use actixweb_email_newsletter::startup::run;
use actixweb_email_newsletter::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool};
use std::net::TcpListener;


                                          


#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
   let subscriber = get_subscriber("actixweb_email_newsletter".into(), "info".into(), std::io::stdout);
   init_subscriber(subscriber);
   

   let configuration = get_configuration().expect("could not read configs");
   println!("HOST = {}", configuration.database.host);
println!("PORT = {}", configuration.database.port);
println!("USER = {}", configuration.database.username);
println!("DB   = {}", configuration.database.database_name);
println!("SSL  = {}", configuration.database.require_ssl);
println!(
    "ENV = {}",
    std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "NOT SET".into())
);
   let connection_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());// `connect_lazy_with` instead of `connect_lazy`
   let address = format!("{}:{}",configuration.application.host, configuration.application.port);
   let listener = TcpListener::bind(address)?;
   run(listener,connection_pool)?.await
}


