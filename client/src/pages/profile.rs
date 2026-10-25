use yew::prelude::*;

use crate::components::avatar::Avatar;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::components::post_list::{PostList, PostListFilter};
use crate::middlewares::context::use_user_context;
use crate::routes::AppRoute;
use crate::services::profile::{follow, get, unfollow};

#[derive(Clone, PartialEq, Eq)]
pub enum ProfileTab {
    ByAuthor,
    FavoritedBy,
}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub display_name: String,
    pub tab: ProfileTab,
}

#[function_component(Profile)]
pub fn profile(props: &Props) -> Html {
    let profile = {
        let display_name = props.display_name.clone();
        use_async(async move { get(display_name).await })
    };
    let follow = {
        let profile = profile.clone();
        let display_name = props.display_name.clone();
        use_async(async move {
            if let Some(resp) = &profile.data
                && resp.data.following {
                    return unfollow(display_name).await;
                }
            follow(display_name).await
        })
    };

    let user_ctx = use_user_context();
    let is_current_user =
        user_ctx.is_authenticated() && user_ctx.display_name == props.display_name;

    {
        let profile = profile.clone();
        use_effect_with(
            props.display_name.clone(),
            move |_| {
                profile.run();
                || ()
            },
        );
    }

    {
        let profile = profile.clone();
        // Key on the async state, not the handle: yew 0.23 handles compare equal across renders.
        let follow = follow.clone();
        use_effect_with(
            (follow.data.clone(), follow.error.clone(), follow.loading),
            move |_| {
                if let Some(resp) = &follow.data {
                    profile.update(resp.clone());
                }
                || ()
            },
        );
    }

    let onclick = {
        Callback::from(move |_| {
            follow.run();
        })
    };

    if let Some(profile) = &profile.data {
        let profile = &profile.data;
        html! {
            <div>
                <div class="bg-gray-100 shadow-lg rounded-lg p-6 my-4">
                    <div class="mx-auto">
                        <div>
                            <div class="col-xs-12 col-md-10 mx-auto">
                                <Avatar name={profile.display_name.clone()} url={profile.profile_image_url.clone()} class="w-20 h-20 mx-auto mb-4 text-2xl" />
                                <h4 class="text-2xl font-bold text-center">{&profile.display_name}</h4>
                                <p class="text-gray-600 text-center">
                                    {
                                        if let Some(bio) = &profile.biography {
                                            html! { bio }
                                        } else {
                                            html! {}
                                        }
                                    }
                                </p>
                                {
                                    if is_current_user {
                                        html! {
                                            <div class="text-gray-600 text-center mt-4">
                                                <Link<AppRoute>
                                                    to={AppRoute::Settings}
                                                    classes="bg-indigo-600 hover:bg-indigo-700 text-white py-2 px-4 rounded-full text-sm">
                                                    { "Edit Profile" }
                                                </Link<AppRoute>>
                                            </div>
                                        }
                                    } else {
                                        html! {
                                            <button class="text-center block mx-auto mt-4 bg-indigo-600 hover:bg-indigo-700 text-white py-2 px-4 rounded-full text-sm" onclick={onclick}>
                                                { if profile.following { "Unfollow" } else { "Follow" } }
                                            </button>
                                        }
                                    }
                                }
                            </div>
                        </div>
                    </div>
                </div>
                <div class="mx-auto">
                    <div>
                        <div class="col-xs-12 md:col-md-10 mx-auto">
                            <div class="mt-4 px-4">
                                <ul class="flex space-x-4">
                                    <li>
                                        <Link<AppRoute>
                                            to={AppRoute::Profile { display_name: profile.display_name.clone() }}
                                            classes={if props.tab == ProfileTab::ByAuthor { "text-indigo-600 font-semibold border-b-2 border-indigo-600" } else { "text-gray-400" }}>
                                            { "My Articles" }
                                        </Link<AppRoute>>
                                    </li>
                                    <li>
                                        <Link<AppRoute>
                                            to={AppRoute::ProfileFavorite { display_name: profile.display_name.clone() }}
                                            classes={if props.tab != ProfileTab::ByAuthor { "text-indigo-600 font-semibold border-b-2 border-indigo-600" } else { "text-gray-400" }}>
                                            { "Favorited Articles" }
                                        </Link<AppRoute>>
                                    </li>
                                </ul>
                            </div>
                            {
                                match props.tab {
                                    ProfileTab::ByAuthor => {
                                        html! { <PostList filter={PostListFilter::ByAuthor(profile.display_name.clone())} /> }
                                    }
                                    ProfileTab::FavoritedBy => {
                                        html! { <PostList filter={PostListFilter::FavoritedBy(profile.display_name.clone())} /> }
                                    }
                                }
                            }
                        </div>
                    </div>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}
