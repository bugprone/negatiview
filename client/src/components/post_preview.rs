use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;
use crate::services::post::{favorite, unfavorite};
use crate::types::post::PostDto;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub post: PostDto,
}

#[function_component(PostPreview)]
pub fn post_preview(props: &Props) -> Html {
    let post = use_state(|| props.post.clone());
    let favorite = {
        let post = post.clone();
        use_async(async move {
            if post.favorited {
                unfavorite(post.slug.clone()).await
            } else {
                favorite(post.slug.clone()).await
            }
        })
    };

    {
        let post = post.clone();
        let favorite = favorite.clone();
        use_effect_with_deps(
            move |favorite| {
                if let Some(post_dto) = &favorite.data {
                    post.set(post_dto.data.clone());
                }
                || ()
            },
            favorite,
        )
    }

    let favorite_button_class = if post.favorited {
        "text-yellow-500"
    } else {
        "text-gray-500"
    };

    let onclick = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            favorite.run();
        })
    };

    html! {
        <div class="bg-indigo-50 rounded-lg shadow-lg p-6 my-4">
            <div class="flex items-center mb-4">
                <img src={post.author.profile_image_url.clone()} alt="Author Image" class="w-10 h-10 rounded-full mr-2" />
                <div>
                    <div class="text-indigo-600 hover:underline">
                        <Link<AppRoute> to={AppRoute::Profile { display_name: post.author.display_name.clone() }}>
                            { &post.author.display_name }
                        </Link<AppRoute>>
                    </div>
                    <span class="text-gray-500 text-sm">
                        { &post.created_at.format("%B %e, %Y") }
                    </span>
                </div>
                <div class="ml-auto">
                    <button class={favorite_button_class} onclick={onclick}>
                        <i class="fas fa-heart mr-1"></i> { post.favorites_count }
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
                {for post.tags.iter().map(|tag| {
                    html! {
                        <li class="mr-2 inline-flex items-center px-2 py-1 text-sm font-medium text-indigo-400 bg-indigo-100 rounded-full" key={ (&tag).to_string() }>
                            { format!("#{}", &tag) }
                        </li>
                    }
                })}
            </ul>
        </div>
    }
}
