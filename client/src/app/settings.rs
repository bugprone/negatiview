use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::app::middleware::context::use_user_context;
use crate::app::middleware::request::{request_get, request_put};
use crate::types::user::{UserDtoWrapper, UserUpdateDto, UserUpdateDtoWrapper};

#[function_component(Settings)]
pub fn setting() -> Html {
    let user_ctx = use_user_context();
    let update_info = use_state(UserUpdateDto::default);
    let password = use_state(String::default);
    let user = use_async_with_options(
        async move {
            request_get::<UserDtoWrapper>("/me".to_string()).await
        },
        UseAsyncOptions::enable_auto(),
    );
    let update = {
        let update_info = update_info.clone();
        let password = password.clone();
        use_async(async move {
            let mut request = UserUpdateDtoWrapper {
                data: (*update_info).clone(),
            };
            if !(*password).is_empty() {
                request.data.password = Some((*password).clone());
            }
            request_put::<UserUpdateDtoWrapper, UserDtoWrapper>("/me".to_string(), request)
                .await
        })
    };

    {
        let user = user.clone();
        let update_info = update_info.clone();
        use_effect_with_deps(
            move |user| {
                if let Some(user) = &user.data {
                    update_info.set(UserUpdateDto {
                        email: user.data.email.clone(),
                        display_name: user.data.display_name.clone(),
                        password: None,
                        // image: user.data.image.clone().unwrap_or_default(),
                        // bio: user.data.bio.clone().unwrap_or_default(),
                    });
                }
                || ()
            },
            user,
        );
    }

    {
        let user_ctx = user_ctx.clone();
        let update = update.clone();
        use_effect_with_deps(
            move |update| {
                if let Some(user) = &update.data {
                    user_ctx.login(user.data.clone());
                }
                || ()
            },
            update,
        );
    }

    let onsubmit = {
        let update = update.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            update.run();
        })
    };

    // let oninput_image = {
    //     let update_info = update_info.clone();
    //     Callback::from(move |e: InputEvent| {
    //         let input: HtmlInputElement = e.target_unchecked_into();
    //         let mut info = (*update_info).clone();
    //         info.image = input.value();
    //         update_info.set(info);
    //     })
    // };

    let oninput_display_name = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.display_name = input.value();
            update_info.set(info);
        })
    };

    // let oninput_bio = {
    //     let update_info = update_info.clone();
    //     Callback::from(move |e: InputEvent| {
    //         let input: HtmlInputElement = e.target_unchecked_into();
    //         let mut info = (*update_info).clone();
    //         info.bio = input.value();
    //         update_info.set(info);
    //     })
    // };

    let oninput_email = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.email = input.value();
            update_info.set(info);
        })
    };

    let oninput_password = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    html! {
        <div class="max-w-md mx-auto mt-12">
            <h1 class="text-center text-xl font-semibold">{ "My Settings" }</h1>
            <form onsubmit={onsubmit} class="mt-4">
                <div class="mb-3">
                    <label for="display_name" class="block text-sm font-medium text-gray-700">{"Display Name"}</label>
                    <input
                        class="mt-1 p-2 block w-full border rounded-md shadow-sm focus:ring focus:ring-indigo-300 focus:outline-none"
                        type="text"
                        placeholder="Display Name"
                        value={update_info.display_name.clone()}
                        oninput={oninput_display_name}
                    />
                </div>
                <div class="mb-3">
                    <label for="email" class="block text-sm font-medium text-gray-700">{ "Email" }</label>
                    <input
                        class="mt-1 p-2 block w-full border rounded-md shadow-sm focus:ring focus:ring-indigo-300 focus:outline-none"
                        type="email"
                        placeholder="Email"
                        value={update_info.email.clone()}
                        oninput={oninput_email}
                    />
                </div>
                <div class="mb-3">
                    <label for="password" class="block text-sm font-medium text-gray-700">{ "New Password"}</label>
                    <input
                        class="mt-1 p-2 block w-full border rounded-md shadow-sm focus:ring focus:ring-indigo-300 focus:outline-none"
                        type="password"
                        placeholder="New Password"
                        value={(*password).clone()}
                        oninput={oninput_password}
                    />
                </div>
                <div class="mb-3 flex justify-center">
                    <button class="px-4 py-2 bg-indigo-600 border rounded-md text-white hover:bg-indigo-700 focus:ring focus:ring-indigo-300 focus:outline-none"
                        type="submit"
                        disabled={user.loading || update.loading}
                    >
                        {"Update Settings"}
                    </button>
                </div>
            </form>
        </div>
    }
}
