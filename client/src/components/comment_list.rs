use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::components::comment::Comment;
use crate::components::new_comment::NewComment;
use crate::middlewares::context::use_user_context;
use crate::routes::AppRoute;
use crate::services::comment::get;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub post_id: String,
}

#[function_component(CommentList)]
pub fn comment_list(props: &Props) -> Html {
    let user_ctx = use_user_context();
    let comment_list = {
        let post_id = props.post_id.clone();
        use_async_with_options(
            async move { get(post_id).await },
            UseAsyncOptions::enable_auto(),
        )
    };

    let callback_added = {
        let comment_list = comment_list.clone();
        Callback::from(move |_| {
            comment_list.run();
        })
    };

    let callback_deleted = {
        let comment_list = comment_list.clone();
        Callback::from(move |_| {
            comment_list.run();
        })
    };

    if let Some(comment_list) = &comment_list.data {
        html! {
            <div class="mt-4">
                <div>
                    {for comment_list.data.comments.iter().map(|comment_dto| {
                        html! {
                            <Comment
                                slug={props.post_id.clone()}
                                comment={comment_dto.clone()}
                                callback={callback_deleted.clone()} />
                        }
                    })}
                </div>
                {
                    if user_ctx.is_authenticated() {
                        html! {
                            <div>
                                <NewComment
                                    post_id={props.post_id.clone()}
                                    callback={callback_added} />
                            </div>
                        }
                    } else {
                        html! {
                            <p>
                                <Link<AppRoute> to={AppRoute::Login} classes="nav-link">
                                    { "Login" }
                                </Link<AppRoute>>
                                { " or " }
                                <Link<AppRoute> to={AppRoute::SignUp} classes="nav-link">
                                    { "Sign up" }
                                </Link<AppRoute>>
                                { " to add comments on this post." }
                            </p>
                        }
                    }
                }
            </div>
        }
    } else {
        html! {}
    }
}
