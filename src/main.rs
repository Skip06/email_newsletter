use actixweb_email_newsletter::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
   // // run().await? // wrong cause Tries to `.await` the `Result<Server, Error>` — **wrong**, `Result` is not a future
   //  run("http://localhost:8000")?.await    //?` unwraps the `Result` → gives a `Server`, then `.await` runs the server future — **correct

   // we need to pass a listener inside run but in production we dont want a random port 
   let listener = TcpListener::bind("localhost:8000")?;
   run(listener)?.await
}


