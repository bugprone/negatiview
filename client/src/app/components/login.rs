use web_sys::{HtmlInputElement};
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::app::middleware::context::{use_user_context};
use crate::app::middleware::request::{request_post};
use crate::types::user::{LoginDto, LoginDtoWrapper, UserDtoWrapper};

#[function_component(Login)]
pub fn login_page() -> Html {
    let user_ctx = use_user_context();
    let login_info = use_state(LoginDto::default);

    let login = {
        let login_info = login_info.clone();
        use_async(async move {
            request_post::<LoginDtoWrapper, UserDtoWrapper>(
                "/login".to_string(),
                LoginDtoWrapper {
                    data: (*login_info).clone(),
                },
            ).await
        })
    };

    use_effect_with_deps(
        move |login| {
            if let Some(resp) = &login.data {
                user_ctx.login(resp.data.clone());
            }
            || ()
        },
        login.clone(),
    );

    let onsubmit = {
        let login = login.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            login.run();
        })
    };

    let oninput_email = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.email = input.value();
            login_info.set(info);
        })
    };

    html! {
        <div class="max-w-md mx-auto mt-12">
            <h1 class="text-center text-xl font-semibold">{ "Login" }</h1>
            <form onsubmit={ onsubmit }>
                <div class="mb-4">
                    <label for="email" class="block text-sm font-medium text-gray-700">{ "Email" }</label>
                    <input
                        class="mt-1 p-2 border rounded w-full"
                        type="email"
                        value={ login_info.email.clone() }
                        oninput={ oninput_email }
                        />
                </div>
                <div class="mb-6 flex justify-center">
                    <button type="submit" class="px-4 py-2 bg-indigo-600 border rounded-md text-white hover:bg-indigo-700 focus:ring focus:ring-indigo-300 focus:outline-none">
                        { "Login" }
                    </button>
                </div>
            </form>
        </div>
    }
}
