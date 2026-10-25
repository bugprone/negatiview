use std::sync::{LazyLock, RwLock};

use gloo::storage::{LocalStorage, Storage};
use gloo_net::http::{Method, RequestBuilder};
use serde::{de::DeserializeOwned, Serialize};

use crate::middlewares::error::{Error, ErrorInfo};

/// Same-origin: `trunk serve` proxies `/api` to the server in development and the
/// server itself serves the bundle in production.
const API_ROOT: &str = "/api";
const TOKEN_KEY: &str = "access_token";

pub static TOKEN: LazyLock<RwLock<Option<String>>> =
    LazyLock::new(|| RwLock::new(LocalStorage::get(TOKEN_KEY).ok()));

pub fn get_token() -> Option<String> {
    let token_lock = TOKEN.read();
    token_lock.unwrap().clone()
}

pub fn set_token(token: Option<String>) {
    if let Some(token) = token.clone() {
        LocalStorage::set(TOKEN_KEY, token).expect("Failed to set token");
    } else {
        LocalStorage::delete(TOKEN_KEY)
    }
    let mut token_lock = TOKEN.write().unwrap();
    *token_lock = token.clone();
}

async fn request<B, T>(method: Method, url: String, body: B) -> Result<T, Error>
where
    B: Serialize + std::fmt::Debug,
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    let allow_body = method == Method::POST || method == Method::PUT;
    let url = format!("{}{}", API_ROOT, url);
    let mut builder = RequestBuilder::new(&url)
        .method(method)
        .header("Content-Type", "application/json");

    if let Some(token) = get_token() {
        builder = builder.header("Authorization", &format!("Bearer {token}"));
    }

    let request = if allow_body {
        builder.json(&body).map_err(|_| Error::BadRequest)?
    } else {
        builder.build().map_err(|_| Error::BadRequest)?
    };

    let response = request.send().await.map_err(|_| Error::BadRequest)?;

    if response.ok() {
        return match response.json::<T>().await {
            Ok(data) => {
                log::debug!("Got data: {:?}", data);
                Ok(data)
            }
            Err(_) => Err(Error::DeserializationError),
        };
    }

    match response.status() {
        401 => Err(Error::Unauthorized),
        403 => Err(Error::Forbidden),
        404 => Err(Error::NotFound),
        500 => Err(Error::InternalServerError),
        422 => match response.json::<ErrorInfo>().await {
            Ok(info) => Err(Error::UnprocessableEntity(info)),
            Err(_) => Err(Error::DeserializationError),
        },
        _ => Err(Error::BadRequest),
    }
}

pub async fn request_get<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(Method::GET, url, ()).await
}

pub async fn request_post<B, T>(url: String, body: B) -> Result<T, Error>
where
    B: Serialize + std::fmt::Debug,
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(Method::POST, url, Some(body)).await
}

pub async fn request_put<B, T>(url: String, body: B) -> Result<T, Error>
where
    B: Serialize + std::fmt::Debug,
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(Method::PUT, url, Some(body)).await
}

pub async fn request_delete<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(Method::DELETE, url, ()).await
}
