use actix_web::{ HttpResponse, web};
use sqlx::PgPool;
use sqlx::types::Uuid;
use sqlx::types::chrono::Utc;

use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(
    form: web::Form<FormData>, // extracts data from incoming form
    connection_pool: web::Data<PgPool> //extracts data from application state
) //You don’t have to manually write code to parse the user's form data,
//and you don’t have to manually write code to fetch the database connection.
//  actix-web looks at the types you wrote (web::Form and web::Data) and automatically "extracts" them for you before running your handler's internal logic.
-> HttpResponse {
    // Let's generate a random unique identifier
    let request_id = Uuid::new_v4();

 /*   We are using the info_span! macro to create a new span and attach some values to its context: request_id,
form.email and form.name.
We are not using string interpolation anymore: tracing allows us to associate structured information to
our spans as a collection of key-value pairs2. We can explicitly name them (e.g. subscriber_email for
form.email) or implicitly use the variable name as key (e.g. the isolated request_id is equivalent to re-
quest_id = request_id).
Notice that we prefixed all of them with a % symbol: we are telling tracing to use their Display implement-
ation for logging purposes. You can find more details on the other available options in their documentation.
info_span returns the newly created span, but we have to explicitly step into it using the .enter() method
to activate it.
.enter() returns an instance of Entered, a guard: as long the guard variable is not dropped all downstream
spans and log events will be registered as children of the entered sp */
    // Spans, like logs, have an associated level
    // `info_span` creates a span at the info-level
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
        );
        // Using `enter` in an async function is a recipe for disaster! cause async works gets mixed within a span
        // Bear with me for now, but don't do this at home.
        // See the following section on `Instrumenting Futures`
    let _request_span_guard = request_span.enter();
        // [...]
        // `_request_span_guard` is dropped at the end of `subscribe`
        // That's when we "exit" the span




    let query_span = tracing::info_span!("saving the new subscriber details in database.",%request_id );






    

    match sqlx::query!(r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#, Uuid::new_v4(),form.email,form.name, Utc::now())
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.
    .execute(connection_pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!("request_id {} new subscriber details saved in db", request_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("request_id {} error when inserting to db -> {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }

}






/*
When a user visits /subscriptions, actix-web prepares to call your subscribe function. It looks at the parameters and notices you asked for web::Data<PgConnection>.

It immediately runs a quick background check: "Let me look at my type-map dictionary. Do I have anything filed under the key PgConnection?"
It finds it, safely converts the data back from a generic blob (Any) into your specific PgConnection wrapper, and passes it directly into your handler's _connection variable
*/

/*
web::Data only gives you read-only, shared references (&PgConnection). It never gives you a mutable reference (&mut).

sqlx demands: Give me an &mut PgConnection or I won't compile! cause of lock concurrency 
actix-web replies: I can only give you an &PgConnection!

Naive solution => Mutexlock o run a query, a worker would have to "lock" the connection, turn it into an &mut, run the query, and unlock it
                but now if multilple qry comes at same time others have to wait
ultimate sol => Connecction Pooling 
                When your application starts up and reads PgPool::connect(), 
                it doesn't just open one communication line. It looks at its configuration and says,
                "I'm going to open up a fleet of 10 distinct network connections to Postgres right now."
It logs into your Postgres Docker container 10 times, establishes 10 unique, active network sockets (TCP pipes), and holds them in a local list (an array or vector) inside your computer's RAM
*/
