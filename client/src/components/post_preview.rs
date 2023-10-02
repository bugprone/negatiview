use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::middlewares::request::{request_delete, request_post};
use crate::routes::AppRoute;
use crate::types::post::Post;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub post: Post,
}

#[function_component(PostPreview)]
pub fn post_preview(props: &Props) -> Html {
    let post = use_state(|| props.post.clone());
    let post_starred = {
        let post = post.clone();
        use_async(async move {
            if post.starred {
                request_delete::<Post>(format!("/posts/{}/star", post.slug)).await
            } else {
                request_post::<(), Post>(format!("/posts/{}/star", post.slug), ()).await
            }
        })
    };

    {
        let post = post.clone();
        let post_starred = post_starred.clone();
        use_effect_with_deps(
            move |post_starred| {
                if let Some(data) = &post_starred.data {
                    post.set(data.clone());
                }
                || ()
            },
            post_starred,
        )
    }

    let star_button_class = if post.starred {
        "text-yellow-500"
    } else {
        "text-gray-500"
    };

    let onclick = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            post_starred.run();
        })
    };

    html! {
        <div class="bg-white rounded-lg shadow-lg p-6 my-4">
            <div class="flex items-center mb-4">
                <img src={post.author.profile_image_url.clone()} alt="Author Image" class="w-8 h-8 rounded-full mr-2" />
                <div class="info">
                    <div class="text-indigo-600 hover:underline">
                        // <Link<Route> to={Route::Profile { display_name: post.author.display_name.clone() }}>
                        //     { &post.author.display_name }
                        // </Link<Route>>
                    </div>
                    <span class="text-gray-500">
                        { &post.created_at.format("%B %e, %Y") }
                    </span>
                </div>
                <div class="ml-auto">
                    <button class={star_button_class} onclick={onclick}>
                        <i class="fas fa-star mr-1"></i> { post.starred_count }
                    </button>
                </div>
            </div>
            <h1 class="text-2xl font-bold mb-2">
                <div class="text-indigo-600 hover:underline">
                    <Link<AppRoute> to={AppRoute::Post { slug: post.slug.clone() }}>
                        { &post.title }
                    </Link<AppRoute >>
                </div>
            </h1>
            <p class="text-gray-600 mb-2">{ &post.description }</p>
            <span class="text-gray-500">
                <Link<AppRoute> to={AppRoute::Post { slug: post.slug.clone() }}>
                    { "Read more..." }
                </Link<AppRoute >>
            </span>
            <ul class="tag-list mt-4">
                {for post.tag_list.iter().map(|tag| {
                    html! {
                        <li class="tag-default tag-pill tag-outline bg-gray-200 text-gray-600 px-2 py-1 rounded-full mr-2" key={ (&tag).to_string() }>
                            { &tag }
                        </li>
                    }
                })}
            </ul>
        </div>
    }
}
