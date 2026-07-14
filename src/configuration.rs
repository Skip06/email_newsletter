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

use env_logger::Env;
use serde::Deserialize;
use secrecy::Secret;
use secrecy::ExposeSecret;

#[derive(Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
pub port: u16,
pub host: String,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub port: u16,
    pub database_name: String,
    pub password: Secret<String>,
    pub host: String,
}
impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password.expose_secret(), self.host, self.port, self.database_name
        ))
    }
    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password.expose_secret(), self.host, self.port
        ))
    }
}
//
// "config" job is to read external files (like YAML or TOML) and translate them into a structured format that Rust understands.

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("could not get current dir");
    let configuration_dir= base_path.join("configuration");
    //detect the running env default is local 
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .expect("could not parse APP_ENVIRONMENT");
    let environment_filename = format!("{}.yaml",environment.as_str());
    let settings = config::Config::builder() //The config crate uses a design pattern called a Builder. Think of this like an assembly line. We are creating a brand new assembly line that is going to construct our configuration object piece-by-piece
        .add_source(config::File::from(
            configuration_dir.join("base.yaml")
            
        ))
        .add_source(config::File::from(
            configuration_dir.join(environment_filename)
        ))
        .build()?; //now execute read, parse
    settings.try_deserialize::<Settings>()
}

pub enum Environment{
    Local,
    Production,
}

impl Environment{
    pub fn as_str(&self) -> &'static str{ // in future we need production.yaml how to get production from Environment::Production
        match self{
            Environment::Local => "local",
            Environment::Production => "production",
            
        }
    }
}

impl TryFrom<String> for Environment {      // from string to type Environment.
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. \
                Use either `local` or `production`.", other
            )),
        }
    }
    }

/*
  SQLX GUARENTTES COMPILE TIME SAFETY AND SQLX::QUERY!() IS A COMPILE TIME MACRO WHICH reaches out to your running Docker PostgreSQL database
  while the compiler is running to check if your SQL statement is valid! It asks the database: "Hey, does a table named subscriptions actually exist?
  And does it have email and name columns?" AND TO CHECK THOSE THINGS IT NEEDS THE CONNECTION_URL AT COMPILE TIME AS IT DOESNOT KNOW ABOUT YAML'S EXITSTENCE
  THE CREATORS OF THIS MACRO MADE IT TO LOOK FOR A .ENV FILE TO GET IT . SO SOLUTION => .ENV FILE 

  AND CONFIGURATION.RS HAS THE CCODE WHICCH  only runs after the binary has finished compiling and is actively running in memory (Runtime). FROM CONFIGURAION.YAML

  Think of the .env file as a developer tool pass required to successfully build and test the project on your machine. 
  Think of configuration.yaml as the actual navigation system your web application uses to survive and operate in the real world once it's turned on
  
 */


 //READ PAGE 126 FOR BETTER UNDERSTANDING OF SECRECY AND THE REASON FOR WRAPPING THE WHOLE CONNECTION STRING AND DISPLAY TRAIT . 