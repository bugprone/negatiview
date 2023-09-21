use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::app::middleware::error::Error;
use crate::app::middleware::request::{get_token, request_get, set_token};
use crate::types::user::{UserInfo, UserInfoWrapper};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let user_ctx = use_state(UserInfo::default);
    let current_user =
        use_async(async move {
            request_get::<UserInfoWrapper>("/users".to_string()).await
            }
        );

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
        use_effect_with_deps(
            move |current_user| {
                if let Some(resp) = &current_user.data {
                    user_ctx.set(resp.user_info.clone());
                }

                if let Some(error) = &current_user.error {
                    match error {
                        Error::Unauthorized | Error::Forbidden => set_token(None),
                        _ => {},
                    }
                }
                || {}
            },
            current_user,
        )
    }

    html! {
        <ContextProvider<UseStateHandle<UserInfo>> context={user_ctx}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<UserInfo>>>
    }
}
