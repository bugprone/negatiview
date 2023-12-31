use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::pagination::Pagination;
use crate::components::post_preview::PostPreview;
use crate::services::post::{all, by_author, by_tag, favorited_by, feed};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PostListFilter {
    All,
    ByAuthor(String),
    ByTag(String),
    Feed,
    FavoritedBy(String),
}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub filter: PostListFilter,
}

#[function_component(PostList)]
pub fn post_list(props: &Props) -> Html {
    let current_page = use_state(|| usize::default());
    let post_list = {
        let filter = props.filter.clone();
        let current_page = current_page.clone();

        use_async(async move {
            match filter {
                PostListFilter::All => { all(*current_page).await }
                PostListFilter::ByAuthor(author) => { by_author(author, *current_page).await }
                PostListFilter::ByTag(tag) => { by_tag(tag, *current_page).await }
                PostListFilter::FavoritedBy(author) => { favorited_by(author, *current_page).await }
                PostListFilter::Feed => { feed().await }
            }
        })
    };

    {
        let current_page = current_page.clone();
        use_effect_with(
            props.filter.clone(),
            move |_| {
                current_page.set(0);
                || ()
            }
        );
    }

    {
        let post_list = post_list.clone();
        use_effect_with(
            (props.filter.clone(), *current_page),
            move |_| {
                post_list.run();
                || ()
            }
        )
    }

    let callback = {
        let current_page = current_page.clone();
        use_callback(
            (),
            move |page, _| {
                current_page.set(page);
            },
        )
    };

    if let Some(resp) = &post_list.data {
        if !resp.data.posts.is_empty() {
            html! {
                <div class="container px-4">
                    {
                        for resp.data.posts.iter().map(|post| {
                            html! { <PostPreview post = { post.clone() } /> }
                        })
                    }
                    <Pagination
                        total = { resp.data.count }
                        current_page = { *current_page }
                        callback = { callback } />
                </div>
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
