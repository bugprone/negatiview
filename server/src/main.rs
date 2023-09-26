use clap::Parser;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions};
use std::{
    sync::Arc,
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
};
use server::route::create_router;
use server::config::{AppState, Config, Opt};

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

    let app = create_router(Arc::new(AppState {
        db: pool.clone(),
        env: config.clone(),
    }), opt);

    log::info!("listening on http://{:?}", socket_addr);

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server")
}
