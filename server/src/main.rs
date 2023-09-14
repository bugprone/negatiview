use clap::Parser;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;
use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
};

use crate::router::create_router;

mod handler;
mod model;
mod router;
mod schema;

pub struct AppState {
    db: Pool<Postgres>,
}

#[derive(Parser, Debug)]
#[clap(name = "server")]
pub struct Opt {
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,

    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,

    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    #[clap(long = "static-dir", default_value = "./dist")]
    static_dir: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            format!("{}, hyper=info, mio=info", opt.log_level),
        )
    }

    tracing_subscriber::fmt::init();

    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => panic!("Error connecting to database: {}", e),
    };

    let socket_addr = SocketAddr::from((
        IpAddr::from_str(&opt.addr).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    let app_state = Arc::new(AppState { db: pool.clone() });
    let app = create_router(app_state, opt);

    log::info!("listening on http://{:?}", socket_addr);

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server")
}
