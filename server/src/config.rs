use clap::Parser;
use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,

    pub access_token_private_key: String,
    pub access_token_public_key: String,
    pub access_token_expires_in: i64,
    pub access_token_max_age: i64,

    pub refresh_token_private_key: String,
    pub refresh_token_public_key: String,
    pub refresh_token_expires_in: i64,
    pub refresh_token_max_age: i64,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");

        let access_token_private_key = std::env::var("ACCESS_TOKEN_PRIVATE_KEY")
            .expect("ACCESS_TOKEN_PRIVATE_KEY must be set");
        let access_token_public_key =
            std::env::var("ACCESS_TOKEN_PUBLIC_KEY").expect("ACCESS_TOKEN_PUBLIC_KEY must be set");
        let access_token_expires_in =
            std::env::var("ACCESS_TOKEN_EXPIRES_IN").expect("ACCESS_TOKEN_EXPIRES_IN must be set");
        let access_token_max_age =
            std::env::var("ACCESS_TOKEN_MAX_AGE").expect("ACCESS_TOKEN_MAX_AGE must be set");

        let refresh_token_private_key = std::env::var("REFRESH_TOKEN_PRIVATE_KEY")
            .expect("REFRESH_TOKEN_PRIVATE_KEY must be set");
        let refresh_token_public_key = std::env::var("REFRESH_TOKEN_PUBLIC_KEY")
            .expect("REFRESH_TOKEN_PUBLIC_KEY must be set");
        let refresh_token_expires_in = std::env::var("REFRESH_TOKEN_EXPIRES_IN")
            .expect("REFRESH_TOKEN_EXPIRES_IN must be set");
        let refresh_token_max_age =
            std::env::var("REFRESH_TOKEN_MAX_AGE").expect("REFRESH_TOKEN_MAX_AGE must be set");

        Config {
            database_url,
            redis_url,
            access_token_private_key,
            access_token_public_key,
            access_token_expires_in: access_token_expires_in.parse::<i64>().unwrap(),
            access_token_max_age: access_token_max_age.parse::<i64>().unwrap(),
            refresh_token_private_key,
            refresh_token_public_key,
            refresh_token_expires_in: refresh_token_expires_in.parse::<i64>().unwrap(),
            refresh_token_max_age: refresh_token_max_age.parse::<i64>().unwrap(),
        }
    }
}

pub struct AppState {
    pub db: Pool<Postgres>,
    pub env: Config,
    pub redis_client: redis::Client,
}

#[derive(Parser, Debug)]
#[clap(name = "server")]
pub struct Opt {
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    pub addr: String,

    #[clap(short = 'p', long = "port", default_value = "8080")]
    pub port: u16,

    #[clap(short = 'l', long = "log", default_value = "debug")]
    pub log_level: String,

    #[clap(long = "static-dir", default_value = "./dist")]
    pub static_dir: String,
}
