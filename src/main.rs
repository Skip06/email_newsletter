use actixweb_email_newsletter::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
   // run().await? // wrong cause Tries to `.await` the `Result<Server, Error>` — **wrong**, `Result` is not a future
    run()?.await    //?` unwraps the `Result` → gives a `Server`, then `.await` runs the server future — **correct
}


