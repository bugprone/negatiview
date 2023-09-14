use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/health")]
    Healthcheck,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <h1> { "Hello Client" } </h1> },
        Route::Healthcheck => html! { <Healthcheck /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[function_component(Healthcheck)]
fn hello_server() -> Html {
    let data = use_state(|| None);

    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::get("/api/health").send().await.unwrap();
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

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
