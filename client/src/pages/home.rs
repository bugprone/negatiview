use yew::prelude::*;

use crate::components::banner::Banner;
use crate::components::post_list::{PostList, PostListFilter};
use crate::components::tag::Tags;
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

#[function_component(MainView)]
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
        <div class="container mx-auto">
            <h1 class="text-xl font-semibold px-4"> { "Posts" } </h1>
            <div class="mt-4 px-4">
                <ul class="flex space-x-4">
                    { global_feed_tab(tab.clone()) }
                    {
                        if user_ctx.is_authenticated() {
                            your_feed_tab(tab.clone())
                        } else {
                            html! {}
                        }
                    }
                    { tag_filter_tab(tab.clone(), props) }
                </ul>
            </div>

            <PostList filter = {(*filter).clone()} />
        </div>
    }
}

fn your_feed_tab(tab: UseStateHandle<Tab>) -> Html {
    let (onclick, class) = get_tab_msg_class(tab, Tab::Feed);

    html! {
        <li class={class}>
            <a href="" onclick={onclick}>
                { "Your Feed" }
            </a>
        </li>
    }
}

fn global_feed_tab(tab: UseStateHandle<Tab>) -> Html {
    let (onclick, class) = get_tab_msg_class(tab, Tab::All);

    html! {
        <li class={class}>
            <a href="" onclick={onclick}>
                { "Global Feed" }
            </a>
        </li>
    }
}

fn tag_filter_tab(tab: UseStateHandle<Tab>, props: &Props) -> Html {
    if let Some(tag) = &props.tag {
        let (onclick, class) = get_tab_msg_class(tab, Tab::Tag);

        html! {
            <li class={class}>
                <a href="" onclick={onclick}>
                    <i class="ion-pound"></i>
                    { &tag }
                </a>
            </li>
        }
    } else {
        html! {}
    }
}

fn get_tab_msg_class(current_tab: UseStateHandle<Tab>, tab: Tab) -> (Callback<MouseEvent>, String) {
    let class = if *current_tab == tab {
        "text-indigo-600 font-semibold border-b-2 border-indigo-600".to_string()
    } else {
        "text-gray-400".to_string()
    };

    let callback = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if *current_tab != tab {
                current_tab.set(tab.clone());
            }
        })
    };

    (callback, class)
}

#[function_component(Home)]
pub fn home() -> Html {
    let tag: UseStateHandle<Option<String>> = use_state(|| None);
    let callback = {
        let tag = tag.clone();
        Callback::from(move |t| {
            tag.set(Some(t));
        })
    };

    html! {
        <div>
            <Banner />
            <div class="container grid grid-cols-12 gap-4 mx-auto py-8">
                <div class="col-span-9">
                    <MainView tag={(*tag).clone()} />
                </div>
                <div class="col-span-3">
                    <h1 class="text-xl font-semibold mb-4"> { "Popular Tags" } </h1>
                    <Tags {callback} />
                </div>
            </div>
        </div>
    }
}
