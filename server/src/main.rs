use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
    sync::Arc,
};

use clap::Parser;
use dotenv::dotenv;
use redis::Client;
use sqlx::postgres::PgPoolOptions;

use server::config::{AppState, Config, Opt};
use server::routes::create_router;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::init();
    let opt = Opt::parse();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            format!("{}, hyper=info, mio=info", opt.log_level),
        )
    }

    tracing_subscriber::fmt::init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => pool,
        Err(err) => panic!("Error connecting to database: {err}"),
    };

    let redis_client = match Client::open(config.redis_url.to_owned()) {
        Ok(client) => client,
        Err(err) => panic!("Error connecting to redis: {err}"),
    };

    let socket_addr = SocketAddr::from((
        IpAddr::from_str(&opt.addr).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    let app = create_router(
        Arc::new(AppState {
            db: pool.clone(),
            env: config.clone(),
            redis_client: redis_client.clone(),
        }),
        opt,
    );

    log::info!(
        "🚀 negatiview server started successfully on http://{:?}",
        socket_addr
    );

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server")
}
