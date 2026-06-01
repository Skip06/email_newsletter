//`tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.\

use std::net::TcpListener;

#[tokio::test]
async fn health_check_status(){

    let address = spawn_app();//its like running cargo run to revive the server for full integration testing 

    let client = reqwest::Client::new();  //reqwest is like axios which we should add as a dev-dependency so it doesnot come on final application binary.
    let response = client
                    .get(&format!("{}/health_check", &address))
                    .send()
                    .await.expect("client could not send or connect");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String{
   // we bind it ourselves and then just make actixweb listen to the socket instead of binding. 
   let listener = TcpListener::bind("localhost:0").expect("counld not bind");
   let port = listener.local_addr().unwrap().port(); // local_addr returns a Result of Ok(Struct Socket) which contains ip and port. 
   
   //to test we want the os to bind at a port at runtime cause hardcoding the port is not ideal in test (book). but now the get req still gets to 8000 and test fails cause app might be spawned at random port
   let server = actixweb_email_newsletter::run(listener).expect("the binding failed"); 

   //this also runs like in as of a thread in background but when the runtime finishes work it dies so this also dies with the runtime cause this spawn() doesnot know when to stop
   let _ = tokio::spawn(server);  


   //we will return the application address to the caller
   format!("http://localhost:{}", port)
   
}