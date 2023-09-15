use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/health")]
    Healthcheck,
    #[at("/users")]
    Users,
    #[at("/posts")]
    Posts,
    #[at("/posts/new")]
    NewPost,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <h1> { "Welcome to Negatiview" } </h1> },
        Route::Healthcheck => html! { <Healthcheck /> },
        Route::Users => html! { <Users /> },
        Route::Posts => html! { <Posts /> },
        Route::NewPost => html! { <NewPost /> },
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

#[function_component(Users)]
fn users_server() -> Html {
    let data = use_state(|| None);

    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::get("/api/users").send().await.unwrap();
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

#[function_component(Posts)]
fn posts_server() -> Html {
    let data = use_state(|| None);

    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::get("/api/posts").send().await.unwrap();
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
                <div class="label"> { "No server response" } </div>
            }
        }
        Some(Ok(data)) => {
            html! {
                <div class="label"> { "Server response: " }{ data } </div>
            }
        }
        Some(Err(err)) => {
            html! {
                <div class="label"> { "Server error: " }{ err } </div>
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

#[function_component(NewPost)]
fn new_post_server() -> Html {
    let title = use_state(|| String::new());
    let content = use_state(|| String::new());

    let title_value = title.clone();
    let content_value = content.clone();

    let create_post = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();

        let request_data = CreatePostRequest {
            title: title.clone().to_string(),
            content: content.clone().to_string(),
        };

        spawn_local(async move {
            let req = Request::post("/api/posts")
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
    });

    html! {
        <div class="container-fluid mt-3">
            <form onsubmit={ create_post }>
                <div class="mb-3">
                    <label for="title" class="form-label">{ "Title" }</label>
                    <input type="text" class="form-control" id="title" value={ title_value.to_string() } />
                </div>
                <div class="mb-3">
                    <label for="content" class="form-label">{ "Content" }</label>
                    <textarea id="content" class="form-control" rows="3" value={ content_value.to_string() } />
                </div>
                <div>
                    <button type="submit" class="btn btn-primary mb-3">{ "Create Post" }</button>
                </div>
            </form>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
