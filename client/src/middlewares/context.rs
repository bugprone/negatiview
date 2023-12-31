use std::fmt;
use std::ops::Deref;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::middlewares::request::set_token;
use crate::routes::AppRoute;
use crate::types::user::UserDto;

pub struct UserUseStateHandle {
    data: UseStateHandle<UserDto>,
    navigator: Navigator,
}

impl UserUseStateHandle {
    pub fn login(&self, value: UserDto) {
        set_token(Some(value.access_token.clone()));
        self.data.set(value);

        self.navigator.push(&AppRoute::Home);
    }

    pub fn login_without_redirection(&self, value: UserDto) {
        set_token(Some(value.access_token.clone()));
        self.data.set(value);
    }

    pub fn logout(&self) {
        set_token(None);
        self.data.set(UserDto::default());

        self.navigator.push(&AppRoute::Home);
    }
}

impl Deref for UserUseStateHandle {
    type Target = UserDto;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Clone for UserUseStateHandle {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            navigator: self.navigator.clone(),
        }
    }
}

impl PartialEq for UserUseStateHandle {
    fn eq(&self, other: &Self) -> bool {
        *self.data == *other.data
    }
}

impl fmt::Debug for UserUseStateHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserUseStateHandle")
            .field("user", &self.data)
            .finish()
    }
}

#[hook]
pub fn use_user_context() -> UserUseStateHandle {
    let user = use_context::<UseStateHandle<UserDto>>().unwrap();
    let navigator = use_navigator().unwrap();

    UserUseStateHandle {
        data: user,
        navigator,
    }
}
