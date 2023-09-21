use yew::prelude::*;
use yew_hooks::prelude::*;
use web_sys::HtmlInputElement;

use crate::app::middleware::context::use_user_context;
use crate::app::middleware::request::request_post;
use crate::types::user::{SignUpRequest, UserInfoWrapper};

#[function_component(SignUp)]
pub fn sign_up_page() -> Html {
    let user_ctx = use_user_context();
    let sign_up_info = use_state(SignUpRequest::default);
    let sign_up = {
        let sign_up_info = sign_up_info.clone();
        use_async(async move {
            request_post::<SignUpRequest, UserInfoWrapper>(
                "/users".to_string(),
                (*sign_up_info).clone(),
            ).await
        })
    };

    use_effect_with_deps(
        move |sign_up| {
            if let Some(resp) = &sign_up.data {
                user_ctx.login(resp.user_info.clone());
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

    let oninput_first_name = {
        let register_req = sign_up_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_req).clone();
            info.first_name = input.value();
            register_req.set(info);
        })
    };

    let oninput_last_name = {
        let register_req = sign_up_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_req).clone();
            info.last_name = input.value();
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
        <div class="max-w-md mx-auto mt-12">
            <h1 class="text-center text-xl font-semibold">{ "Sign Up" }</h1>
            <form onsubmit={ onsubmit } class="mt-4">
                <div class="mb-3">
                    <label for="email" class="block text-sm font-medium text-gray-700">{ "Email" }</label>
                    <input
                        class="mt-1 p-2 block w-full border rounded-md shadow-sm focus:ring focus:ring-indigo-300 focus:outline-none"
                        type="email"
                        value={ sign_up_info.email.clone() }
                        oninput={ oninput_email }
                    />
                </div>
                <div class="mb-3">
                    <label for="first_name" class="block text-sm font-medium text-gray-700">{ "First Name" }</label>
                    <input
                        class="mt-1 p-2 block w-full border rounded-md shadow-sm focus:ring focus:ring-indigo-300 focus:outline-none"
                        type="text"
                        value={ sign_up_info.first_name.clone() }
                        oninput={ oninput_first_name }
                    />
                </div>
                <div class="mb-3">
                    <label for="last_name" class="block text-sm font-medium text-gray-700">{ "Last Name" }</label>
                    <input
                        class="mt-1 p-2 block w-full border rounded-md shadow-sm focus:ring focus:ring-indigo-300 focus:outline-none"
                        type="text"
                        value={ sign_up_info.last_name.clone() }
                        oninput={ oninput_last_name }
                    />
                </div>
                <div class="mb-3">
                    <label for="display_name" class="block text-sm font-medium text-gray-700">{ "Display Name" }</label>
                    <input
                        class="mt-1 p-2 block w-full border rounded-md shadow-sm focus:ring focus:ring-indigo-300 focus:outline-none"
                        type="text"
                        value={ sign_up_info.display_name.clone() }
                        oninput={ oninput_display_name }
                    />
                </div>
                <div class="mb-3 flex justify-center">
                    <button type="submit" class="px-4 py-2 bg-indigo-600 border rounded-md text-white hover:bg-indigo-700 focus:ring focus:ring-indigo-300 focus:outline-none">
                        { "Sign Up" }
                    </button>
                </div>
            </form>
        </div>
    }
}
