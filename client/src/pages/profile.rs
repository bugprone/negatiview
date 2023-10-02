use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::components::post_list::{PostList, PostListFilter};
use crate::middlewares::context::use_user_context;
use crate::middlewares::request::{request_delete, request_get, request_post};
use crate::routes::AppRoute;
use crate::types::profile::ProfileWrapper;

#[derive(Clone, PartialEq, Eq)]
pub enum ProfileTab {
    ByAuthor,
    FollowedBy,
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
        use_async(async move {
            request_get::<ProfileWrapper>(format!("/profile/{}", display_name)).await
        })
    };
    let follow = {
        let profile = profile.clone();
        let display_name = props.display_name.clone();
        use_async(async move {
            if let Some(resp) = &profile.data {
                if resp.data.following {
                    return request_delete::<ProfileWrapper>(format!(
                        "/profile/{}/follow",
                        display_name
                    ))
                    .await;
                }
            }
            request_post::<(), ProfileWrapper>(format!("/profile/{}/follow", display_name), ())
                .await
        })
    };

    let user_ctx = use_user_context();
    let is_current_user =
        user_ctx.is_authenticated() && user_ctx.display_name == props.display_name;

    {
        let profile = profile.clone();
        use_effect_with_deps(
            move |_| {
                profile.run();
                || ()
            },
            props.display_name.clone(),
        );
    }

    {
        let profile = profile.clone();
        use_effect_with_deps(
            move |follow| {
                if let Some(resp) = &follow.data {
                    profile.update(resp.clone());
                }
            },
            follow.clone(),
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
                <div class="bg-white rounded-lg shadow-lg p-6 my-4">
                    <div class="mx-auto">
                        <div>
                            <div class="col-xs-12 col-md-10 mx-auto">
                                <img src={profile.profile_image_url.clone()} alt={profile.display_name.clone()} class="user-img w-20 h-20 rounded-full mx-auto mb-4" />
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
                                            <div class="action-btn block mx-auto mt-4">
                                                <Link<AppRoute> to={AppRoute::Settings}>
                                                    { "Edit Profile Settings" }
                                                </Link<AppRoute>>
                                            </div>
                                        }
                                    } else {
                                        html! {
                                            <button class="block mx-auto mt-4" onclick={onclick}>
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
                        <div class="col-xs-12 col-md-10 mx-auto">
                            <div class="mt-4">
                                <ul class="nav nav-pills outline-active">
                                    <li class="nav-item">
                                        <div class={if props.tab == ProfileTab::ByAuthor { "nav-link active" } else { "nav-link" }}>
                                            <Link<AppRoute> to={AppRoute::Profile { display_name: profile.display_name.clone() }}>
                                                { "My Articles" }
                                            </Link<AppRoute>>
                                        </div>
                                    </li>
                                    <li class="nav-item">
                                        <div class={if props.tab != ProfileTab::ByAuthor { "nav-link active" } else { "nav-link" }}>
                                            <Link<AppRoute> to={AppRoute::Follow { display_name: profile.display_name.clone() }}>
                                                { "Favorited Articles" }
                                            </Link<AppRoute>>
                                        </div>
                                    </li>
                                </ul>
                            </div>
                            {
                                match props.tab {
                                    ProfileTab::ByAuthor => {
                                        html! { <PostList filter={PostListFilter::ByAuthor(profile.display_name.clone())} /> }
                                    }
                                    ProfileTab::FollowedBy => {
                                        html! { <PostList filter={PostListFilter::StarredBy(profile.display_name.clone())} /> }
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
