use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::types::user::SignUpRequest;

#[function_component(SignUp)]
pub fn sign_up_page() -> Html {
    let sign_up_request = use_state(SignUpRequest::default);

    let onsubmit = {
        let sign_up_req = sign_up_request.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let request_data = (*sign_up_req).clone();

            spawn_local(async move {
                let req = Request::post("/api/users")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&request_data).unwrap())
                    .unwrap();

                let resp = req.send().await.unwrap();

                if resp.ok() {
                    log::info!("{:?}", resp);
                } else {
                    log::error!("{:?}", resp);
                }
            })
        })
    };

    let oninput_email = {
        let register_req = sign_up_request.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_req).clone();
            info.email = input.value();
            register_req.set(info);
        })
    };

    let oninput_first_name = {
        let register_req = sign_up_request.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_req).clone();
            info.first_name = input.value();
            register_req.set(info);
        })
    };

    let oninput_last_name = {
        let register_req = sign_up_request.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_req).clone();
            info.last_name = input.value();
            register_req.set(info);
        })
    };

    let oninput_display_name = {
        let register_req = sign_up_request.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_req).clone();
            info.display_name = input.value();
            register_req.set(info);
        })
    };

    html! {
        <div class="col-md-6 offset-md-3 col-xs-12 mt-3">
            <h1 class="text-xs-center">{ "Sign Up" }</h1>
            <p class="text-xs-center">
                // <Link<Route> to={Route::Login}>
                //     { "Have an account?" }
                // </Link<Route>>
            </p>
            // <ListErrors error={user_register.error.clone()} />
            <form onsubmit={ onsubmit }>
                <div class="mb-3">
                    <label for="email" class="form-label">{ "Email" }</label>
                    <input
                        class="form-control"
                        type="email"
                        value={ sign_up_request.email.clone() }
                        oninput={ oninput_email }
                        />
                </div>
                <div class="mb-3">
                    <label for="first_name" class="form-label">{ "First Name" }</label>
                    <input
                        class="form-control"
                        type="text"
                        value={ sign_up_request.first_name.clone() }
                        oninput={ oninput_first_name }
                        />
                </div>
                <div class="mb-3">
                    <label for="last_name" class="form-label">{ "Last Name" }</label>
                    <input
                        class="form-control"
                        type="text"
                        value={ sign_up_request.last_name.clone() }
                        oninput={ oninput_last_name }
                        />
                </div>
                <div class="mb-3">
                    <label for="display_name" class="form-label">{ "Display Name" }</label>
                    <input
                        class="form-control"
                        type="text"
                        value={ sign_up_request.display_name.clone() }
                        oninput={ oninput_display_name }
                        />
                </div>
                <div class="mb-3 d-flex justify-content-center">
                    <button type="submit" class="btn btn-primary"> { "Sign Up" } </button>
                </div>
            </form>
        </div>
    }
}
