use crate::domain::SubscriberEmail;
use secrecy::ExposeSecret;
use secrecy::Secret;
use std::convert::{TryFrom, TryInto};

#[derive(Clone, serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub email_client: EmailClientSettings,
    pub redis_uri: Secret<String>,
}
#[derive(Clone, serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}
#[derive(Clone, serde::Deserialize)]
pub struct ApplicationSettings {
    // #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
    pub hmac_secret: Secret<String>,
}

#[derive(serde::Deserialize)]
pub struct AwsEnv {
    password: Secret<String>,
    dbname: String,
    port: u16,
    host: String,
    username: String,
}
#[derive(Clone, serde::Deserialize)]
pub struct EmailClientSettings {
    pub base_url: String,
    pub sender_email: String,
    pub authorization_token: Secret<String>,
    pub timeout_milliseconds: u64,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to determine the current directory!");
    let configuration_directory = base_path.join("configuration");

    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    settings.merge(
        config::File::from(configuration_directory.join(environment.as_str())).required(true),
    )?;

    settings.merge(config::Environment::with_prefix("app").separator("__"))?;
    let mut result_settings: Settings = settings.try_into().unwrap();

    if Environment::Production == environment {
        let aws: AwsEnv =
            serde_json::from_str(&std::env::var("ZERO2PRODSVCCLUSTER_SECRET").unwrap()).unwrap();
        result_settings.database.username = aws.username;
        result_settings.database.password = aws.password;
        result_settings.database.port = aws.port;
        result_settings.database.host = aws.host;
        result_settings.database.database_name = aws.dbname;
        result_settings.application.base_url = "https://api.postmarkapp.com/email".to_string();
    }
    Ok(result_settings)
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }
    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
        ))
    }
}

#[derive(PartialEq)]
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not supported environment. Use either 'local' or 'production'.",
                other
            )),
        }
    }
}

impl EmailClientSettings {
    pub fn sender(&self) -> Result<SubscriberEmail, String> {
        SubscriberEmail::parse(self.sender_email.clone())
    }
    pub fn timeout(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.timeout_milliseconds)
    }
}
