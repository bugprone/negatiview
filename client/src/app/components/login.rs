use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;
use crate::types::user::LoginRequest;

#[function_component(Login)]
pub fn login_page() -> Html {
    let navigator = use_navigator().unwrap();
    let cloned_navigator = navigator.clone();

    let login_request = use_state(LoginRequest::default);

    let onsubmit = {
        let sign_up_req = login_request.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let navigator = cloned_navigator.clone();
            let request_data = (*sign_up_req).clone();

            spawn_local(async move {
                let req = Request::post("/api/users/login")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&request_data).unwrap())
                    .unwrap();

                let resp = req.send().await.unwrap();

                if resp.ok() {
                    navigator.push(&Route::Home);
                } else {
                    log::error!("{:?}", resp);
                }
            })
        })
    };

    let oninput_email = {
        let register_req = login_request.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_req).clone();
            info.email = input.value();
            register_req.set(info);
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
                        value={ login_request.email.clone() }
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
