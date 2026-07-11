use actixweb_email_newsletter::startup::run;
use actixweb_email_newsletter::configuration::get_configuration;
use sqlx::{PgPool};
use std::net::TcpListener;
                                                                                                //use env_logger::Env;

use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
   // // run().await? // wrong cause Tries to `.await` the `Result<Server, Error>` — **wrong**, `Result` is not a future
   //  run("http://localhost:8000")?.await    //?` unwraps the `Result` → gives a `Server`, then `.await` runs the server future — **correct

   // `init` does call `set_logger`, so this is all we need to do.
   // We are falling back to printing all logs at info-level or above equivalent ,to RUST_LOG=info cargo run
   // if the RUST_LOG environment variable has not been set.
                                                                                          //env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
   let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));

   let formatting_layer = BunyanFormattingLayer::new(
      "actixweb_email_newsletter".into(),
      // Output the formatted spans to stdout.
      std::io::stdout
   );

      // The `with` method is provided by `SubscriberExt`, an extension
   // trait for `Subscriber` exposed by `tracing_subscriber`
   let subscriber = Registry::default()
   .with(env_filter)
   .with(JsonStorageLayer)
   .with(formatting_layer);
   // `set_global_default` can be used by applications to specify
   // what subscriber should be used to process spans.
   set_global_default(subscriber).expect("Failed to set subscriber");


   let configuration = get_configuration().expect("could not read configs");
   let connection_pool = PgPool::connect(&configuration.database.connection_string()).await.expect("could not connect to postgres");
   let address_port = configuration.app_port;
   let listener = TcpListener::bind(format!("127.0.0.1:{}", address_port))?;
   run(listener,connection_pool)?.await
}


