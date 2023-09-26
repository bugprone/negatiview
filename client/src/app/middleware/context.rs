use std::fmt;
use std::ops::Deref;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::middleware::request::set_token;
use crate::router::Route;
use crate::types::user::UserDto;

pub struct UserUseStateHandle {
    user: UseStateHandle<UserDto>,
    navigator: Navigator,
}

impl UserUseStateHandle {
    pub fn login(&self, value: UserDto) {
        set_token(Some(value.token.clone()));
        self.user.set(value);

        self.navigator.push(&Route::Home);
    }

    pub fn logout(&self) {
        set_token(None);
        self.user.set(UserDto::default());

        self.navigator.push(&Route::Home);
    }
}

impl Deref for UserUseStateHandle {
    type Target = UserDto;

    fn deref(&self) -> &Self::Target {
        &self.user
    }
}

impl Clone for UserUseStateHandle {
    fn clone(&self) -> Self {
        Self {
            user: self.user.clone(),
            navigator: self.navigator.clone(),
        }
    }
}

impl PartialEq for UserUseStateHandle {
    fn eq(&self, other: &Self) -> bool {
        *self.user == *other.user
    }
}

impl fmt::Debug for UserUseStateHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserUseStateHandle")
            .field("user", &self.user)
            .finish()
    }
}

#[hook]
pub fn use_user_context() -> UserUseStateHandle {
    let user = use_context::<UseStateHandle<UserDto>>().unwrap();
    let navigator = use_navigator().unwrap();

    UserUseStateHandle { user, navigator }
}
