use std::sync::RwLock;
use gloo::storage::{LocalStorage, Storage};
use lazy_static::lazy_static;

const TOKEN_KEY: &str = "negatiview.token";

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
