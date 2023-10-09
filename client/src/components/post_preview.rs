use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;
use crate::services::post::{favorite, unfavorite};
use crate::types::post::PostDto;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub post: PostDto,
}

pub struct PostPreview {
    props: Props,
    favorited: bool,
    favorites_count: i64,
}

pub enum Msg {
    Favorite,
    FavoriteSuccess(bool),
}

impl PostPreview {
    fn favorite(&mut self, ctx: &Context<Self>) {
        let post = self.props.post.clone();
        let link = ctx.link().clone();

        spawn_local(async move {
            let result = if post.favorited {
                unfavorite(post.slug.clone()).await
            } else {
                favorite(post.slug.clone()).await
            };

            log::debug!("Favorite result: {:?}", result);
            link.send_message(Msg::FavoriteSuccess(result.is_ok()));
        });
    }
}

impl Component for PostPreview {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: Props { post: ctx.props().post.clone() },
            favorited: ctx.props().post.favorited,
            favorites_count: ctx.props().post.favorites_count,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Favorite => {
                self.favorite(ctx);
            }
            Msg::FavoriteSuccess(success) => {
                log::debug!("Favorite success: {}", success);
                if success {
                    if self.favorited {
                        self.favorites_count -= 1;
                        self.favorited = false;
                    } else {
                        self.favorites_count += 1;
                        self.favorited = true;
                    }
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let post = ctx.props().post.clone();

        html! {
            <div class="bg-indigo-50 rounded-lg shadow-lg p-6 my-4">
                <div class="flex items-center relative mb-4">
                    <img src={post.author.profile_image_url} alt="Author Image" class="w-10 h-10 rounded-full mr-2" />
                    <div>
                        <div class="text-indigo-600 hover:underline">
                            <Link<AppRoute> to={AppRoute::Profile { display_name: post.author.display_name.clone() }}>
                                { post.author.display_name }
                            </Link<AppRoute>>
                        </div>
                        <span class="text-gray-500 text-sm">
                            { format!("{}", &post.created_at.format("%B %e, %Y")) }
                        </span>
                    </div>
                    <div class="absolute top-0 right-0">
                        <button onclick={ctx.link().callback(|_| Msg::Favorite)} class={if self.favorited {
                            "text-red-500 border-2 border-red-500 rounded-full px-2 py-1 inline-flex justify-center items-center"
                        } else {
                            "text-gray-400 border-2 border-gray-400 rounded-full px-2 py-1 inline-flex justify-center items-center"
                        }}>
                            <svg class="w-5 h-5 mr-1 fill-current">
                                <path d="M9.653 16.915l-.005-.003-.019-.010a20.759 20.759 0 01-1.162-.682 22.045 22.045 0 01-2.582-1.900C4.045 12.733 2 10.352 2 7.500a4.5 4.5 0 018-2.828A4.5 4.5 0 0118 7.500c0 2.852-2.044 5.233-3.885 6.820a22.049 22.049 0 01-3.744 2.582l-.019.010-.005.003h-.002a.739.739 0 01-.690.001l-.002-.001z" />
                            </svg>
                            <span class="text-sm">
                                { self.favorites_count }
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
                            <li class="mr-2 inline-flex items-center px-2 py-1 text-sm font-medium text-indigo-400 bg-indigo-100 rounded-full" key={ tag.to_string() }>
                                { format!("#{}", tag) }
                            </li>
                        }
                    })}
                </ul>
            </div>
        }
    }
}
