use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
    sync::Arc,
};

use clap::Parser;
use redis::Client;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::EnvFilter;

use server::config::{AppState, Config, Opt};
use server::routes::create_router;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config = Config::init();
    let opt = Opt::parse();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::new(format!("{},hyper=info,mio=info", opt.log_level))
        }))
        .init();

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

    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .unwrap_or_else(|err| panic!("Unable to bind {socket_addr}: {err}"));

    tracing::info!("negatiview server listening on http://{socket_addr}");

    axum::serve(listener, app)
        .await
        .expect("Unable to start server")
}
