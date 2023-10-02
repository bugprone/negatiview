use std::sync::RwLock;

use gloo::storage::{LocalStorage, Storage};
use lazy_static::lazy_static;
use serde::{de::DeserializeOwned, Serialize};

use crate::middlewares::error::{Error, ErrorInfo};

const API_ROOT: &str = "http://localhost:8081/api";
const TOKEN_KEY: &str = "access_token";

lazy_static! {
    pub static ref TOKEN: RwLock<Option<String>> = {
        if let Ok(token) = LocalStorage::get(TOKEN_KEY) {
            RwLock::new(Some(token))
        } else {
            RwLock::new(None)
        }
    };
}

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

async fn request<B, T>(method: reqwest::Method, url: String, body: B) -> Result<T, Error>
where
    B: Serialize + std::fmt::Debug,
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let url = format!("{}{}", API_ROOT, url);
    let mut builder = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");

    if let Some(token) = get_token() {
        builder = builder.bearer_auth(token);
    }

    if allow_body {
        builder = builder.json(&body);
    }

    let response = builder.send().await;

    if let Ok(data) = response {
        if data.status().is_success() {
            let data: Result<T, _> = data.json::<T>().await;
            if let Ok(data) = data {
                log::debug!("Got data: {:?}", data);
                Ok(data)
            } else {
                Err(Error::DeserializationError)
            }
        } else {
            match data.status().as_u16() {
                401 => Err(Error::Unauthorized),
                403 => Err(Error::Forbidden),
                404 => Err(Error::NotFound),
                500 => Err(Error::InternalServerError),
                422 => {
                    let data: Result<ErrorInfo, _> = data.json::<ErrorInfo>().await;
                    if let Ok(data) = data {
                        Err(Error::UnprocessableEntity(data))
                    } else {
                        Err(Error::DeserializationError)
                    }
                }
                _ => Err(Error::BadRequest),
            }
        }
    } else {
        Err(Error::BadRequest)
    }
}

pub async fn request_get<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwest::Method::GET, url, ()).await
}

pub async fn request_post<B, T>(url: String, body: B) -> Result<T, Error>
where
    B: Serialize + std::fmt::Debug,
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwest::Method::POST, url, Some(body)).await
}

pub async fn request_put<B, T>(url: String, body: B) -> Result<T, Error>
where
    B: Serialize + std::fmt::Debug,
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwest::Method::PUT, url, Some(body)).await
}

pub async fn request_delete<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwest::Method::DELETE, url, ()).await
}
