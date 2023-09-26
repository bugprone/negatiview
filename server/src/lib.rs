use clap::Parser;
use sqlx::{Pool, Postgres};

pub mod dto;
pub mod model;
pub mod handler;
pub mod router;
pub mod schema;

pub struct AppState {
    pub db: Pool<Postgres>,
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
