use std::path::PathBuf;
use std::sync::Arc;

use axum::{http::StatusCode, middleware, Router, routing::get, routing::post};
use axum::body::{Body, boxed};
use axum::http::Response;
use tokio::fs;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use crate::config::{AppState, Opt};
use crate::handlers::health_check;
use crate::handlers::post::{feed_list, get_post, new_post, post_list};
use crate::handlers::profile::{follow_user, get_user_profile, unfollow_user};
use crate::handlers::user::{login, me, sign_up, update_me};
use crate::middlewares::auth::auth;

pub fn create_router(app_state: Arc<AppState>, opt: Opt) -> Router {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .route(
                    "/health",
                    get(health_check)
                )
                .nest(
                    "/user",
                    Router::new()
                        .route(
                            "/",
                            get(me).put(update_me)
                                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
                        )
                        .route(
                            "/login",
                            post(login)
                        )
                        .route(
                            "/sign_up",
                            post(sign_up)
                        )
                )
                .nest(
                    "/profile",
                    Router::new()
                        .route(
                            "/:display_name",
                            get(get_user_profile)
                                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
                        )
                        .route(
                            "/:display_name/follow",
                            post(follow_user).delete(unfollow_user)
                                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
                        )
                )
                .nest(
                    "/posts",
                    Router::new()
                        .route(
                            "/",
                            get(post_list).post(new_post)
                                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
                        )
                        .route(
                            "/feed",
                            get(feed_list)
                                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
                        )
                        .route(
                            "/:slug",
                            get(get_post)
                                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
                        )
                )
        )
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
