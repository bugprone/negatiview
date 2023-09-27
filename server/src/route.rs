use axum::body::{boxed, Body};
use axum::http::Response;
use axum::{http::StatusCode, routing::get, routing::post, Router, middleware};
use std::path::PathBuf;
use std::sync::Arc;
use axum::routing::put;
use tokio::fs;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use crate::auth::auth;
use crate::config::{AppState, Opt};
use crate::handler::*;

pub fn create_router(app_state: Arc<AppState>, opt: Opt) -> Router {
    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/me", get(me)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route("/api/me", put(update_me)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route("/api/users", get(user_list))
        .route("/api/users", post(new_user))
        .route("/api/login", post(login))
        .route("/api/posts", get(post_list))
        .route("/api/posts", post(new_post))
        .fallback_service(get(|req| async move {
            match ServeDir::new(&opt.static_dir).oneshot(req).await {
                Ok(res) => {
                    let status = res.status();
                    match status {
                        StatusCode::NOT_FOUND => {
                            let index_path = PathBuf::from(&opt.static_dir).join("index.html");
                            let index_content = match fs::read_to_string(index_path).await {
                                Ok(index_content) => index_content,
                                Err(_) => {
                                    return Response::builder()
                                        .status(StatusCode::NOT_FOUND)
                                        .body(boxed(Body::from("index file not found")))
                                        .unwrap()
                                }
                            };

                            Response::builder()
                                .status(StatusCode::OK)
                                .body(boxed(Body::from(index_content)))
                                .unwrap()
                        }
                        _ => res.map(boxed),
                    }
                }
                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("error: {err}"))))
                    .expect("error response"),
            }
        }))
        .layer(CorsLayer::permissive())
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(app_state)
}
