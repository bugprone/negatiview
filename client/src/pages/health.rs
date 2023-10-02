use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(Healthcheck)]
pub fn hello_server() -> Html {
    let data = use_state(|| None);

    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::get("/services/health").send().await.unwrap();
                    let result = {
                        if !resp.ok() {
                            Err(format!(
                                "Error fetching data {} ({})",
                                resp.status(),
                                resp.status_text()
                            ))
                        } else {
                            resp.text().await.map_err(|err| err.to_string())
                        }
                    };
                    data.set(Some(result));
                });
            }

            || ()
        });
    }

    match data.as_ref() {
        None => {
            html! {
                <div> { "No server response" } </div>
            }
        }
        Some(Ok(data)) => {
            html! {
                <div> { "Server response: " }{ data } </div>
            }
        }
        Some(Err(err)) => {
            html! {
                <div> { "Server error: " }{ err } </div>
            }
        }
    }
}
