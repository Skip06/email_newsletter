use std::net::TcpListener;

use actixweb_email_newsletter::configuration::{DatabaseSettings, get_configuration};
use sqlx::{Connection, Executor, PgConnection, PgPool, Row};
use uuid::Uuid;
use actixweb_email_newsletter::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
/*
Given that we are then going to need that very same connection pool in sub-
scribe_returns_a_200_for_valid_form_data to perform our SELECT query, it makes sense to generalise
spawn_app: instead of returning a raw String, we will give the caller a struct, TestApp. TestApp will hold
both the address of our test application instance and a handle to the connection pool, simplifying the
arrange steps in our test cases
*/
static TRACING: Lazy<()> = Lazy::new(|| { //static makes it exits for entire lifetime of program
    let subscriber = get_subscriber("test".into(), "debug".into(),std::io::stdout);
    init_subscriber(subscriber); //but if i run cargo test subscriber would be init for all tests causing calling the internal global fn more than once so panics => use "once_cell"
});
pub struct TestApp {
    pub address: String,
    pub connection_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);
    
    let listener = TcpListener::bind("localhost:0").expect("counld not bind");
    let port = listener.local_addr().unwrap().port(); // local_addr returns a Result of Ok(Struct Socket) which contains ip and port.

    let mut configuration =
        get_configuration().expect("could not get the Settings config from config.yaml");
    //WILL BE CREATING TEMPORARY RANDOM TABLE FOR TEST
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;

    //we will return the application address to the caller
    let address = format!("http://localhost:{}", port);
    let server = actixweb_email_newsletter::run(listener, connection_pool.clone())
        .expect("the binding failed");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        connection_pool,
    }
}

#[tokio::test]
async fn health_check_status() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &test_app.address))
        .send()
        .await
        .expect("client could not send or connect");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

//now will test if user sends name and email correctly
#[tokio::test]
async fn subscriber_returns_valid_formdata() {
    // this test case needs to veryfy if the data is in the db or not

    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscription", &test_app.address))
        .body(body)
        .header("Content-Type", "application/x-www-form-urlencoded") // this is telling actix to treat this body as HTML FormData
        .send()
        .await
        .expect("could not sent the post request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query("SELECT name, email FROM subscriptions")
        .fetch_one(&test_app.connection_pool)
        .await
        .expect("could not fetch the saved subscription");

    assert_eq!(saved.get::<String, _>("email"), "ursula_le_guin@gmail.com");
    assert_eq!(saved.get::<String, _>("name"), "le guin");
}

//THE TESTS ARE WORKING BUT WHILE TESTING , THE TEST IS STORING THIGNS IN MAIN db BUT NOT CLEANING IT UP
// • wrap the whole test in a SQL transaction and rollback at the end of it;  // GOOD BUT TRICKY TO DO HERE DUE TO CONNECTIONPOOL
//• spin up a brand-new logical database for each integration test TEST IT THEN DROP IT
#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscription", &test_app.address))
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

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");
    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}
