use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(Posts)]
pub fn posts_server() -> Html {
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
struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

#[function_component(NewPost)]
pub fn new_post_server() -> Html {
    let title = use_state(|| String::new());
    let content = use_state(|| String::new());

    let title_value = title.clone();
    let content_value = content.clone();

    let oninput_title = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            title.set(input.value());
        })
    };

    let oninput_content = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            content.set(input.value());
        })
    };

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
        <div class="col-md-6 offset-md-3 col-xs-12 mt-3">
            <h1 class="text-xs-center">{ "New Post" }</h1>
            <form onsubmit={ create_post }>
                <div class="mb-3">
                    <label for="title" class="form-label">{ "Title" }</label>
                    <input type="text" class="form-control" id="title" value={ title_value.clone().to_string() } oninput={oninput_title} />
                </div>
                <div class="mb-3">
                    <label for="content" class="form-label">{ "Content" }</label>
                    <textarea id="content" class="form-control" rows="3" value={ content_value.clone().to_string() } oninput={oninput_content} />
                </div>
                <div class="mb-3 d-flex justify-content-center">
                    <button type="submit" class="btn btn-primary">{ "Create Post" }</button>
                </div>
            </form>
        </div>
    }
}
