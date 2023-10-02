use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::routes::AppRoute;

#[derive(Debug, Serialize, Deserialize)]
struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

#[function_component(NewPost)]
pub fn new_post_page() -> Html {
    let navigator = use_navigator().unwrap();
    let cloned_navigator = navigator.clone();

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
        let navigator = cloned_navigator.clone();

        let request_data = CreatePostRequest {
            title: title.clone().to_string(),
            content: content.clone().to_string(),
        };

        spawn_local(async move {
            let req = Request::post("/services/posts")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&request_data).unwrap())
                .unwrap();

            let resp = req.send().await.unwrap();

            if resp.ok() {
                log::info!("{:?}", resp);
                navigator.push(&AppRoute::Home);
            } else {
                log::error!("{:?}", resp);
            }
        })
    });

    html! {
        <div class="max-w-md mx-auto mt-12">
            <h1 class="text-center text-xl font-semibold">{ "New Post" }</h1>
            <form onsubmit={ create_post }>
                <div class="mb-3">
                    <label for="title" class="block text-sm font-medium text-gray-700">{ "Title" }</label>
                    <input
                        type="text"
                        id="title"
                        class="mt-1 p-2 block w-full border rounded-md shadow-sm focus:ring focus:ring-indigo-300 focus:outline-none"
                        value={ title_value.clone().to_string() }
                        oninput={oninput_title}
                    />
                </div>
                <div class="mb-3">
                    <label for="content" class="block text-sm font-medium text-gray-700">{ "Content" }</label>
                    <textarea
                        id="content"
                        rows="3"
                        class="mt-1 p-2 block w-full border rounded-md shadow-sm focus:ring focus:ring-indigo-300 focus:outline-none"
                        value={ content_value.clone().to_string() }
                        oninput={oninput_content}
                    />
                </div>
                <div class="mb-3 flex justify-center">
                    <button type="submit" class="px-4 py-2 bg-indigo-600 border rounded-md text-white hover:bg-indigo-700 focus:ring focus:ring-indigo-300 focus:outline-none">
                        { "Create Post" }
                    </button>
                </div>
            </form>
        </div>
    }
}
