//`tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.\

#[tokio::test]
async fn health_check_status(){

    spawn_app();//its like running cargo run to revive the server for full integration testing 

    let client = reqwest::Client::new();  //reqwest is like axios which we should add as a dev-dependency so it doesnot come on final application binary.
    let response = client
                    .get("http://localhost:8000/health_check")
                    .send()
                    .await.expect("client could not send or connect");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app(){
   
   // actixweb_email_newsletter::run().await  // but now spawn app wont return cause server is just running . test wont complete ever on its own
   let server = actixweb_email_newsletter::run().expect("the binding failed"); //.expect() works on a Result i.e returns Ok value and if Err panics with a msg

   //this also runs like in as of a thread in background but when the runtime finishes work it dies so this also dies with the runtime cause this spawn() doesnot know when to stop
   let _ = tokio::spawn(server);  
   
}