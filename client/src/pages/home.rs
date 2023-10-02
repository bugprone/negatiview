use yew::prelude::*;

use crate::components::banner::Banner;
use crate::components::post_list::{PostList, PostListFilter};
use crate::middlewares::context::use_user_context;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub tag: Option<String>,
}

#[derive(PartialEq, Eq, Clone)]
pub enum Tab {
    All,
    Feed,
    Tag,
}

#[function_component(MainPage)]
pub fn main_page(props: &Props) -> Html {
    let user_ctx = use_user_context();
    let tab = use_state(|| {
        if user_ctx.is_authenticated() {
            Tab::Feed
        } else {
            Tab::All
        }
    });

    let filter = use_state(|| {
        if user_ctx.is_authenticated() {
            PostListFilter::Feed
        } else {
            PostListFilter::All
        }
    });

    {
        let tab = tab.clone();
        let filter = filter.clone();
        use_effect_with_deps(
            move |tag| {
                if let Some(tag) = &tag {
                    tab.set(Tab::Tag);
                    filter.set(PostListFilter::ByTag(tag.clone()));
                }
                || ()
            },
            props.tag.clone(),
        );
    }

    {
        let filter = filter.clone();
        use_effect_with_deps(
            move |(tab, tag)| {
                match tab {
                    Tab::All => filter.set(PostListFilter::All),
                    Tab::Feed => filter.set(PostListFilter::Feed),
                    Tab::Tag => {
                        if let Some(tag) = tag {
                            filter.set(PostListFilter::ByTag(tag.clone()));
                        }
                    }
                }
                || ()
            },
            ((*tab).clone(), props.tag.clone()),
        );
    }

    html! {
        <div class="col-span-9">
            <div class="grid grid-cols-12 gap-4">
                <div class="col-span-12 sm:col-span-12 md:col-span-9">
                    <div class="posts">
                        <ul class="flex flex-wrap space-x-2">
                        </ul>
                    </div>
                </div>
            </div>
            <PostList filter = {(*filter).clone()} />
        </div>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let tag: UseStateHandle<Option<String>> = use_state(|| None);
    let _callback = {
        let tag = tag.clone();
        Callback::from(move |t| {
            tag.set(Some(t));
        })
    };

    html! {
        <div>
            <Banner />
            <div class="container mx-auto py-8">
                <div class="grid grid-cols-12 gap-4">
                    <MainPage tag={(*tag).clone()} />
                    <div class="col-span-12 sm:col-span-12 md:col-span-3">
                        <div class="max-w-screen-lg mx-auto p-4">
                            <h1 class="text-3xl font-semibold mb-4"> { "Sidebar" } </h1>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
