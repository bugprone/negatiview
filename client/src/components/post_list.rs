use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::list_pagination::ListPagination;
use crate::components::post_preview::PostPreview;
use crate::middlewares::pagination::limit;
use crate::middlewares::request::request_get;
use crate::types::post::Posts;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PostListFilter {
    All,
    ByAuthor(String),
    ByTag(String),
    StarredBy(String),
    Feed,
}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub filter: PostListFilter,
}

#[function_component(PostList)]
pub fn post_list(props: &Props) -> Html {
    let current_page = use_state(|| 0u32);
    let post_list = {
        let filter = props.filter.clone();
        let current_page = current_page.clone();

        use_async(async move {
            match filter {
                PostListFilter::All => {
                    request_get::<Posts>(format!("/posts?{}", limit(10, *current_page))).await
                }
                PostListFilter::ByAuthor(author) => {
                    request_get::<Posts>(format!(
                        "/posts?author={}&{}",
                        author,
                        limit(10, *current_page)
                    ))
                    .await
                }
                PostListFilter::ByTag(tag) => {
                    request_get::<Posts>(format!("/posts?tag={}&{}", tag, limit(10, *current_page)))
                        .await
                }
                PostListFilter::StarredBy(author) => {
                    request_get::<Posts>(format!(
                        "/posts?starred_by={}&{}",
                        author,
                        limit(10, *current_page)
                    ))
                    .await
                }
                PostListFilter::Feed => {
                    request_get::<Posts>(format!("/posts/feed?{}", limit(10, 0))).await
                }
            }
        })
    };

    {
        let current_page = current_page.clone();
        use_effect_with_deps(
            move |_| {
                current_page.set(0);
                || ()
            },
            props.filter.clone(),
        );
    }

    {
        let post_list = post_list.clone();
        use_effect_with_deps(
            move |_| {
                post_list.run();
                || ()
            },
            (props.filter.clone(), *current_page),
        )
    }

    let callback = {
        let current_page = current_page.clone();
        use_callback(
            move |page, _| {
                current_page.set(page);
            },
            (),
        )
    };

    if let Some(post_list) = &post_list.data {
        if !post_list.posts.is_empty() {
            html! {
                <>
                    { for post_list.posts.iter().map(|post| {
                        html! { <PostPreview post = {post.clone()} /> }
                    })}
                    <ListPagination
                        total = { post_list.count }
                        current_page = { *current_page }
                        callback = { callback } />
                </>
            }
        } else {
            html! {
                <div class="text-center mt-4"> { "No posts" } </div>
            }
        }
    } else {
        html! {
            <div class="text-center mt-4"> { "Loading" } </div>
        }
    }
}

// #[derive(Debug, Deserialize)]
// struct Post {
//     pub title: String,
//     pub content: String,
// }
//
// #[derive(Debug, Deserialize)]
// struct PostListResponse {
//     pub posts: Vec<Post>,
// }
// #[function_component(PostList)]
// pub fn post_list() -> Html {
//     let data = use_state(|| None);
//
//     {
//         let data = data.clone();
//         use_effect(move || {
//             if data.is_none() {
//                 spawn_local(async move {
//                     let resp = Request::get("/services/posts").send().await.unwrap();
//                     let result = {
//                         if !resp.ok() {
//                             Err(format!(
//                                 "Error fetching data {} ({})",
//                                 resp.status(),
//                                 resp.status_text()
//                             ))
//                         } else {
//                             resp.text().await.map_err(|err| err.to_string())
//                         }
//                     };
//                     data.set(Some(result));
//                 });
//             }
//
//             || ()
//         });
//     }
//
//     match data.as_ref() {
//         None => {
//             html! {
//                <div class="text-center mt-4"> { "Loading" } </div>
//             }
//         }
//         Some(Ok(data)) => {
//             let resp: PostListResponse = serde_json::from_str(&data).unwrap();
//
//             html! {
//                 <div class="max-w-screen-lg mx-auto p-4">
//                     <h1 class="text-3xl font-semibold mb-4"> { "Posts" } </h1>
//                     <div class="space-y-4">
//                         { for resp.posts.iter().map(|post| render_post(post)) }
//                     </div>
//                 </div>
//             }
//         }
//         Some(Err(err)) => {
//             html! {
//                 <div class="text-center mt-4 text-red-500"> { format!("Server error: {}", err) } </div>
//             }
//         }
//     }
// }
//
// fn render_post(post: &Post) -> Html {
//     let content_lines: Vec<&str> = post.content.split('\n').collect();
//
//     html! {
//         <div class="p-4 bg-white shadow rounded-lg mb-4">
//             <h3 class="text-xl font-semibold">{ &post.title }</h3>
//             <hr class="my-2" />
//             { for content_lines.iter().map(|line| html! { <p>{ line }</p> }) }
//         </div>
//     }
// }
