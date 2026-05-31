//`tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.\

#[tokio::test]
async fn health_check_status(){

    spawn_app().await.expect("Failed to spawn app"); //its like running cargo run to revive the server for full integration testing 

    let client = reqwest::Client::new();  //reqwest is like axios which we should add as a dev-dependency so it doesnot come on final application binary.
    let response = client
                    .get("localhost:8000/health_check")
                    .send()
                    .await.expect("client could not send or connect");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
                        
    

    
    
    
}

async fn spawn_app()-> Result<(), std::io::Error>{
   
    actixweb_email_newsletter::run().await
}