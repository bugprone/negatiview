use std::path::PathBuf;
use std::sync::Arc;

use axum::{middleware, Router, routing::get, routing::post};
use axum::routing::delete;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

use crate::config::{AppState, Opt};
use crate::handlers::comment::{delete_comment, get_comments, new_comment};
use crate::handlers::health_check;
use crate::handlers::post::{delete_post, favorite_post, feed_list, get_post, new_post, post_list, unfavorite_post, update_post};
use crate::handlers::profile::{follow_user, get_user_profile, unfollow_user};
use crate::handlers::tag::get_tags;
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
                            "/{display_name}",
                            get(get_user_profile)
                                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
                        )
                        .route(
                            "/{display_name}/follow",
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
                        .nest(
                            "/{id}",
                            Router::new()
                                .route("/",
                                       get(get_post).put(update_post).delete(delete_post)
                                           .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
                                )
                                .route("/favorite",
                                       post(favorite_post).delete(unfavorite_post)
                                           .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
                                )
                                .nest(
                                    "/comments",
                                    Router::new()
                                        .route("/",
                                               get(get_comments).post(new_comment))
                                                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
                                        .route("/{comment_id}",
                                               delete(delete_comment)
                                               .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
                                        )
                                )
                        )
                )
                .route(
                    "/tags",
                    get(get_tags)
                )
        )
        .fallback_service(
            ServeDir::new(&opt.static_dir).fallback(ServeFile::new(
                PathBuf::from(&opt.static_dir).join("index.html"),
            )),
        )
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}
