use actix_web::{ HttpResponse, web};
use sqlx::PgPool;
use sqlx::types::Uuid;
use sqlx::types::chrono::Utc;

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
    println!("HANDLER CALLED");

    let _ = sqlx::query!(r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#, Uuid::new_v4(),form.email,form.name, Utc::now())
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.
    .execute(connection_pool.get_ref())
    .await;

    HttpResponse::Ok().finish()
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
