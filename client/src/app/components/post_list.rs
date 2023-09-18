use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, Html, html, use_effect, use_state};

#[derive(Debug, Deserialize)]
struct Post {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
struct PostListResponse {
    pub posts: Vec<Post>,
}
#[function_component(PostList)]
pub fn post_list() -> Html {
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
               <div class="text-center mt-4 text-red-500"> { "No server response" } </div>
            }
        }
        Some(Ok(data)) => {
            let resp: PostListResponse = serde_json::from_str(&data).unwrap();

            html! {
                <div class="max-w-screen-lg mx-auto mt-8">
                    <h1 class="text-3xl font-semibold mb-4"> { "Posts" } </h1>
                    <div class="space-y-4">
                        { for resp.posts.iter().map(|post| render_post(post)) }
                    </div>
                </div>
            }
        }
        Some(Err(err)) => {
            html! {
                <div class="text-center mt-4 text-red-500"> { format!("Server error: {}", err) } </div>
            }
        }
    }
}

fn render_post(post: &Post) -> Html {
    let content_lines: Vec<&str> = post.content.split('\n').collect();

    html! {
        <div class="p-4 bg-white shadow rounded-lg mb-4">
            <h3 class="text-xl font-semibold">{ &post.title }</h3>
            <hr class="my-2" />
            { for content_lines.iter().map(|line| html! { <p>{ line }</p> }) }
        </div>
    }
}
