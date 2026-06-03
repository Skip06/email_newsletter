/* 
The script was for you (the developer) and your tools; this Rust code is for the application itself.
Before your application can even run, you need a living, breathing database server.
    Your init_db.sh script talks to Docker to turn on the database.
    Then, your terminal tool (sqlx-cli) uses that script's connection string to run migrations and build the empty tables.
Once that script finishes, it dies. It is completely gone. It does not stay alive inside your compiled Rust binary.
Phase B: The Application Runtime (The Rust Code)

Now, imagine a user opens a web browser and types their email to subscribe to your newsletter. Their browser sends a network request to your running Rust application.

Your script is long gone. How does the compiled Rust binary running in memory know where the database is? It can't read your shell script files.

It has to connect to PostgreSQL all by itself. That is why you are writing DatabaseSettings::connection_string(&self). When your web server starts up, it reads configuration.yaml, generates its own connection string using this function,
and opens a permanent line of communication to PostgreSQL so it can save those incoming user emails.

*/

use config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings{
    pub app_port: u16,
    pub database: DatabaseSettings
}

#[derive(Deserialize)]
pub struct DatabaseSettings{
    pub username: String,
    pub port: u16,
    pub database_name: String,
    pub password: String,
    pub host: String
}
impl DatabaseSettings{
    pub fn connection_string(&self) -> String{
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}
//
// "config" job is to read external files (like YAML or TOML) and translate them into a structured format that Rust understands.

pub fn get_configuration() -> Result<Settings, config::ConfigError>{
    let settings = config::Config::builder() //The config crate uses a design pattern called a Builder. Think of this like an assembly line. We are creating a brand new assembly line that is going to construct our configuration object piece-by-piece
        .add_source(config::File::new("configuration.yaml", config::FileFormat::Yaml))
        .build()?;  //now execute read, parse 
    settings.try_deserialize::<Settings>()
}