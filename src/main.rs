use actixweb_email_newsletter::configuration::get_configuration;
use actixweb_email_newsletter::startup::run;
use actixweb_email_newsletter::{get_subscriber, init_subscriber};
use sqlx::{PgPool};
use std::net::TcpListener;


                                          


#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
   let subscriber = get_subscriber("actixweb_email_newsletter".into(), "info".into(), std::io::stdout);
   init_subscriber(subscriber);

   let configuration = get_configuration().expect("could not read configs");
   let connection_pool = PgPool::connect(&configuration.database.connection_string()).await.expect("could not connect to postgres");
   let address_port = configuration.app_port;
   let listener = TcpListener::bind(format!("127.0.0.1:{}", address_port))?;
   run(listener,connection_pool)?.await
}


