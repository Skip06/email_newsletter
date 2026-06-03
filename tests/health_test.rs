//`tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.\

use std::net::TcpListener;

use actixweb_email_newsletter::configuration::get_configuration;
use sqlx::{Connection, PgConnection};

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

//now will test if user sends name and email correctly 
#[tokio::test]
async fn subscriber_returns_valid_formdata(){
    let configuration = get_configuration().expect("could not read configuration");
    let connection_url = configuration.database.connection_string();
    let mut connnection = PgConnection::connect(&connection_url).await.expect("failed to connect to Postgres");
    let address = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
                    .post(&format!("{}/subscription", address))
                    .body(body)
                    .header("Content-Type", "application/x-www-form-urlencoded") // this is telling actix to treat this body as HTML FormData
                    .send()
                    .await.expect("could not sent the post request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("select name, email from subscriptions",)
    .fetch_one(&mut connnection)
    .await.expect("could not fetch the saved subscription");
assert_eq!(saved.email, "ursula_le_guin@gmail.com");
assert_eq!(saved.name, "le guin");

    

    
}


#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];
    for (invalid_body, error_message) in test_cases {

        let response = client
                        .post(&format!("{}/subscription", &app_address))
                        .header("Content-Type", "application/x-www-form-urlencoded")
                        .body(invalid_body)
                        .send()
                        .await
                        .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
        }
}