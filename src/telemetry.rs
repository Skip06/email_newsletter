use tracing::{Subscriber, subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer; // this is for actix's internal log actix's instrument still uses log crate but we removed env_logger() so no one to listen for that  so we need tracing-log crate for log -> tracing
use tracing_subscriber::fmt::MakeWriter;
/// We are using `impl Subscriber` as return type to avoid having to
/// spell out the actual type of the returned subscriber, which is
/// indeed quite complex.
/// We need to explicitly call out that the returned subscriber is
/// `Send` and `Sync` to make it possible to pass it to `init_subscriber`
/// later on.



pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink
    )-> impl Subscriber + Send + Sync
         where 
         // This "weird" syntax is a higher-ranked trait bound (HRTB)
         // It basically means that Sink implements the `MakeWriter`
         Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static, //just says sink is somethign implementing makewriter.
    {
       let env_filter = EnvFilter::try_from_default_env()
             .unwrap_or_else(|_| EnvFilter::new(env_filter));
 
       let formatting_layer = BunyanFormattingLayer::new(
          name,
          // Output the formatted spans to stdout.
          sink
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
    LogTracer::init().expect("Failed to set logger");  //actix web's internal intrumetation is covered with logs but as we removed env_logger no once to listen so LogTracer::init() allows to log -> trace
    set_global_default(subscriber).expect("Failed to set subscriber");
 }
 