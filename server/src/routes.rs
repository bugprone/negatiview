use std::path::PathBuf;
use std::sync::Arc;

use axum::{http::StatusCode, middleware, Router, routing::get, routing::post, routing::put};
use axum::body::{Body, boxed};
use axum::http::Response;
use tokio::fs;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use crate::config::{AppState, Opt};
use crate::handlers::*;
use crate::middlewares::auth::auth;

pub fn create_router(app_state: Arc<AppState>, opt: Opt) -> Router {
    Router::new()
        .route("/services/health", get(health_check))
        .route(
            "/services/me",
            get(me).route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/services/me",
            put(update_me).route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route("/services/users", get(user_list))
        .route("/services/users", post(new_user))
        .route("/services/login", post(login))
        .route("/services/posts", get(post_list))
        .route("/services/posts", post(new_post))
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
