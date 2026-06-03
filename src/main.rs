use actixweb_email_newsletter::startup::run;
use actixweb_email_newsletter::configuration::{self, get_configuration};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
   // // run().await? // wrong cause Tries to `.await` the `Result<Server, Error>` — **wrong**, `Result` is not a future
   //  run("http://localhost:8000")?.await    //?` unwraps the `Result` → gives a `Server`, then `.await` runs the server future — **correct

   let configuration = get_configuration().expect("could not read configs");
   // we need to pass a listener inside run but in production we dont want a random port 
   //let listener = TcpListener::bind("localhost:8000")?; //but this is hardcoded
   let address_port = configuration.app_port;
   let listener = TcpListener::bind(format!("localhost:{}", address_port))?;
   run(listener)?.await
}


