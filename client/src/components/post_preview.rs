use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;
use crate::services::post::{favorite, unfavorite};
use crate::types::post::PostDto;

#[derive(Properties, Clone, PartialEq)]
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
        use_effect_with(
            favorite,
            move |favorite| {
                if let Some(post_dto) = &favorite.data {
                    post.set(post_dto.data.clone());
                }
                || ()
            },
        )
    }

    let onclick = {
        let post = post.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            favorite.run();
            let mut dto = (*post).clone();
            if post.favorited {
                dto.favorited = false;
                dto.favorites_count -= 1;
            } else {
                dto.favorited = true;
                dto.favorites_count += 1;
            }
            post.set(dto);
        })
    };

    html! {
        <div class="bg-indigo-50 rounded-lg shadow-lg p-6 my-4">
            <div class="flex items-center relative mb-4">
                <img src={post.author.profile_image_url.clone()} alt="Author Image" class="w-10 h-10 rounded-full mr-2" />
                <div>
                    <div class="text-indigo-600 hover:underline">
                        <Link<AppRoute> to={AppRoute::Profile { display_name: post.author.display_name.clone() }}>
                            { &post.author.display_name }
                        </Link<AppRoute>>
                    </div>
                    <span class="text-gray-500 text-sm">
                        { format!("{}", &post.created_at.format("%B %e, %Y")) }
                    </span>
                </div>
                <div class="absolute top-0 right-0">
                    <button onclick={onclick} class={if post.clone().favorited {
            "text-red-500 border-2 border-red-500 rounded-full px-2 py-1 inline-flex justify-center items-center"
        } else {
            "text-gray-400 border-2 border-gray-400 rounded-full px-2 py-1 inline-flex justify-center items-center"
        }}>
                        <svg class="w-5 h-5 mr-1 fill-current">
                            <path d="M9.653 16.915l-.005-.003-.019-.01a20.759 20.759 0 01-1.162-.682 22.045 22.045 0 01-2.582-1.9C4.045 12.733 2 10.352 2 7.5a4.5 4.5 0 018-2.828A4.5 4.5 0 0118 7.5c0 2.852-2.044 5.233-3.885 6.82a22.049 22.049 0 01-3.744 2.582l-.019.01-.005.003h-.002a.739.739 0 01-.69.001l-.002-.001z" />
                        </svg>
                        <span class="text-sm">
                        { post.favorites_count }
                        </span>
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
