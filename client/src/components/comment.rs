use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::delete_comment::DeleteComment;

use crate::middlewares::context::use_user_context;
use crate::routes::AppRoute;
use crate::types::comment::CommentDto;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub slug: String,
    pub comment: CommentDto,
    pub callback: Callback<String>,
}

#[function_component(Comment)]
pub fn comment(props: &Props) -> Html {
    let user_ctx = use_user_context();
    let comment = &props.comment;
    let can_delete = user_ctx.is_authenticated() && user_ctx.display_name == comment.author.display_name;

    html! {
        <div class="p-4 border border-gray-200 rounded-md shadow-md mb-4">
            <p class="text-gray-800 mb-4">{ &comment.body }</p>
            <div class="flex items-center space-x-2 text-gray-600">
                <img
                    src={ comment.author.profile_image_url.clone() }
                    class="w-8 h-8 rounded-full"
                    alt={ comment.author.display_name.clone() }
                />
                <Link<AppRoute> classes="font-semibold hover:text-indigo-600" to={AppRoute::Profile { display_name: comment.author.display_name.clone() }}>
                    { &comment.author.display_name }
                </Link<AppRoute>>
                <span class="text-xs">
                    { format!("{}", &comment.created_at.format("%B %e, %Y %H:%M")) }
                </span>
                { if can_delete {
                    html! {
                        <DeleteComment
                            post_id={props.slug.clone()}
                            comment_id={comment.id.clone()}
                            callback={props.callback.clone()}
                            />
                    }
                } else {
                    html! { }
                }}
            </div>
        </div>
    }
}

