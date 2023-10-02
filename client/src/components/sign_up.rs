use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::middlewares::context::use_user_context;
use crate::middlewares::request::request_post;
use crate::routes::AppRoute;
use crate::types::user::{SignUpDto, SignUpDtoWrapper, UserDtoWrapper};

#[function_component(SignUp)]
pub fn sign_up_page() -> Html {
    let user_ctx = use_user_context();
    let sign_up_info = use_state(SignUpDto::default);
    let sign_up = {
        let sign_up_info = sign_up_info.clone();
        use_async(async move {
            request_post::<SignUpDtoWrapper, UserDtoWrapper>(
                "/users".to_string(),
                SignUpDtoWrapper {
                    data: (*sign_up_info).clone(),
                },
            )
            .await
        })
    };

    use_effect_with_deps(
        move |sign_up| {
            if let Some(resp) = &sign_up.data {
                user_ctx.login(resp.data.clone());
            }
            || ()
        },
        sign_up.clone(),
    );

    let onsubmit = {
        let sign_up = sign_up.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            sign_up.run();
        })
    };

    let oninput_email = {
        let register_req = sign_up_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_req).clone();
            info.email = input.value();
            register_req.set(info);
        })
    };

    let oninput_password = {
        let register_req = sign_up_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_req).clone();
            info.password = input.value();
            register_req.set(info);
        })
    };

    let oninput_display_name = {
        let register_req = sign_up_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_req).clone();
            info.display_name = input.value();
            register_req.set(info);
        })
    };

    html! {
        <div class="max-w-md mx-auto mt-12 mb-12">
            <h1 class="text-center text-xl font-semibold">{ "Sign Up" }</h1>
            <form onsubmit={ onsubmit } class="mt-4">
                <div class="mb-4">
                    <label for="email" class="block text-sm font-medium text-gray-700">
                        { "Email" }
                    </label>
                    <input
                        class="mt-1 p-2 block w-full border rounded-md shadow-sm focus:ring focus:ring-indigo-300 focus:outline-none"
                        type="email"
                        value={ sign_up_info.email.clone() }
                        oninput={ oninput_email }
                    />
                </div>
                <div class="mb-4">
                    <label for="password" class="block text-sm font-medium text-gray-700">
                        { "Password" }
                    </label>
                    <input
                        class="mt-1 p-2 block w-full border rounded-md shadow-sm focus:ring focus:ring-indigo-300 focus:outline-none"
                        type="password"
                        value={ sign_up_info.password.clone() }
                        oninput={ oninput_password }
                    />
                </div>
                <div class="mb-4">
                    <label for="display_name" class="block text-sm font-medium text-gray-700">
                        { "Display Name" }
                    </label>
                    <input
                        class="mt-1 p-2 block w-full border rounded-md shadow-sm focus:ring focus:ring-indigo-300 focus:outline-none"
                        type="text"
                        value={ sign_up_info.display_name.clone() }
                        oninput={ oninput_display_name }
                    />
                </div>
                <div class="flex justify-center">
                    <button type="submit" class="w-full px-4 py-2 bg-indigo-600 border rounded-md text-white hover:bg-indigo-700 focus:ring focus:ring-indigo-300 focus:outline-none">
                        { "Sign Up" }
                    </button>
                </div>
                <p class="text-center text-sm">
                    { "Have an account? " }
                    <span class="text-blue-500 hover:underline">
                        <Link<AppRoute> to={AppRoute::Login}>
                            { "Login" }
                        </Link<AppRoute >>
                    </span>
                </p>
            </form>
        </div>
    }
}