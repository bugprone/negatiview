use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::types::user::LoginRequest;

#[function_component(Login)]
pub fn login_page() -> Html {
    let login_request = use_state(LoginRequest::default);

    let onsubmit = {
        let sign_up_req = login_request.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let request_data = (*sign_up_req).clone();

            spawn_local(async move {
                let req = Request::post("/api/users/login")
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
        let register_req = login_request.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_req).clone();
            info.email = input.value();
            register_req.set(info);
        })
    };

    html! {
        <div class="col-md-6 offset-md-3 col-xs-12 mt-3">
            <h1 class="text-xs-center">{ "Login" }</h1>
            <form onsubmit={ onsubmit }>
                <div class="mb-3">
                    <label for="email" class="form-label">{ "Email" }</label>
                    <input
                        class="form-control"
                        type="email"
                        value={ login_request.email.clone() }
                        oninput={ oninput_email }
                        />
                </div>
                <div class="mb-3 d-flex justify-content-center">
                    <button type="submit" class="btn btn-primary"> { "Login" } </button>
                </div>
            </form>
        </div>
    }
}
