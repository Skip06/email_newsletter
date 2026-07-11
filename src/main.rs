use actixweb_email_newsletter::startup::run;
use actixweb_email_newsletter::configuration::get_configuration;
use sqlx::{PgPool};
use std::net::TcpListener;
                                                                                                //use env_logger::Env;

use tracing::{Subscriber, subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer; // this is for actix's internal log actix's instrument still uses log crate but we removed env_logger() so no one to listen for that  so we need tracing-log crate for log -> tracing

/// We are using `impl Subscriber` as return type to avoid having to
/// spell out the actual type of the returned subscriber, which is
/// indeed quite complex.
/// We need to explicitly call out that the returned subscriber is
/// `Send` and `Sync` to make it possible to pass it to `init_subscriber`
/// later on.





                                                                                          //env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
   pub fn get_subscriber(
      name: String,
      env_filter: String,
   )-> impl Subscriber + Send + Sync{
      let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));

   let formatting_layer = BunyanFormattingLayer::new(
      "actixweb_email_newsletter".into(),
      // Output the formatted spans to stdout.
      std::io::stdout
   );

      // The `with` method is provided by `SubscriberExt`, an extension
   // trait for `Subscriber` exposed by `tracing_subscriber`
   let subscriber = Registry::default()   //subscriber is of now 3 layers wrapped but we want Registry which implements Subscriber trait so just return type impl Subscriber
   .with(env_filter)
   .with(JsonStorageLayer)
   .with(formatting_layer);

   return subscriber;
   }

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
   LogTracer::init().expect("Failed to set logger");
   set_global_default(subscriber).expect("Failed to set subscriber");
   }



#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
   let subscriber = get_subscriber("actixweb_email_newsletter".into(), "info".into());
   init_subscriber(subscriber);

   let configuration = get_configuration().expect("could not read configs");
   let connection_pool = PgPool::connect(&configuration.database.connection_string()).await.expect("could not connect to postgres");
   let address_port = configuration.app_port;
   let listener = TcpListener::bind(format!("127.0.0.1:{}", address_port))?;
   run(listener,connection_pool)?.await
}


