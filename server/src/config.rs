use clap::Parser;
use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: i64,
    pub jwt_max_age: i64,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in = std::env::var("JWT_EXPIRES_IN").expect("JWT_EXPIRES_IN must be set");
        let jwt_max_age = std::env::var("JWT_MAX_AGE").expect("JWT_MAX_AGE must be set");
        Config {
            database_url,
            jwt_secret,
            jwt_expires_in: jwt_expires_in.parse::<i64>().unwrap(),
            jwt_max_age: jwt_max_age.parse::<i64>().unwrap(),
        }
    }
}

pub struct AppState {
    pub db: Pool<Postgres>,
    pub env: Config,
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
