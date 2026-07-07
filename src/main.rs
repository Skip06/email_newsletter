use actixweb_email_newsletter::startup::run;
use actixweb_email_newsletter::configuration::get_configuration;
use sqlx::{PgPool};
use std::net::TcpListener;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
   // // run().await? // wrong cause Tries to `.await` the `Result<Server, Error>` — **wrong**, `Result` is not a future
   //  run("http://localhost:8000")?.await    //?` unwraps the `Result` → gives a `Server`, then `.await` runs the server future — **correct

   // `init` does call `set_logger`, so this is all we need to do.
   // We are falling back to printing all logs at info-level or above equivalent ,to RUST_LOG=info cargo run
   // if the RUST_LOG environment variable has not been set.
   env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
   
   let configuration = get_configuration().expect("could not read configs");
   let connection_pool = PgPool::connect(&configuration.database.connection_string()).await.expect("could not connect to postgres");
   let address_port = configuration.app_port;
   let listener = TcpListener::bind(format!("127.0.0.1:{}", address_port))?;
   run(listener,connection_pool)?.await
}


