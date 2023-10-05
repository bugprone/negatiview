use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::middlewares::error::Error;
use crate::middlewares::request::{get_token, set_token};
use crate::services::user::current;
use crate::types::user::UserDto;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let user_ctx = use_state(UserDto::default);
    let current_user =
        use_async(async move {
            current().await
        });

    {
        let current_user = current_user.clone();
        use_mount(move || {
            if get_token().is_some() {
                current_user.run();
            }
        });
    }

    {
        let user_ctx = user_ctx.clone();
        use_effect_with(
            current_user,
            move |current_user| {
                if let Some(resp) = &current_user.data {
                    user_ctx.set(resp.data.clone());
                }

                if let Some(error) = &current_user.error {
                    match error {
                        Error::Unauthorized | Error::Forbidden => set_token(None),
                        _ => {}
                    }
                }
                || {}
            },
        )
    }

    html! {
        <ContextProvider<UseStateHandle<UserDto >> context={user_ctx}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<UserDto >>>
    }
}
