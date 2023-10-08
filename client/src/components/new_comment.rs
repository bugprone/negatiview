use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::show_error::ShowError;
use crate::middlewares::context::use_user_context;
use crate::services::comment::create;
use crate::types::comment::{CommentDto, NewCommentDto};
use crate::types::Wrapper;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub slug: String,
    pub callback: Callback<CommentDto>,
}

#[function_component(NewComment)]
pub fn new_comment(props: &Props) -> Html {
    let user_ctx = use_user_context();
    let new_comment_dto = use_state(NewCommentDto::default);
    let new_comment = {
        let slug = props.slug.clone();
        let new_comment_dto = new_comment_dto.clone();
        use_async(async move {
            create(
                slug,
                Wrapper::<NewCommentDto> { data: (*new_comment_dto).clone() }
            ).await
        })
    };

    let onsubmit = {
        let new_comment = new_comment.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            new_comment.run();
        })
    };

    let oninput = {
        let new_comment_dto = new_comment_dto.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut dto = (*new_comment_dto).clone();
            dto.body = input.value();
            new_comment_dto.set(dto);
        })
    };

    {
        let new_comment_dto = new_comment_dto.clone();
        let callback = props.callback.clone();
        use_effect_with(
            new_comment.clone(),
            move |new_comment| {
                if let Some(comment_dto) = &new_comment.data {
                    new_comment_dto.set(NewCommentDto::default());
                    callback.emit(comment_dto.data.clone());
                }
                || ()
            }
        )
    };

    let button_disabled = new_comment.loading || new_comment_dto.body.is_empty();

    html! {
        <div class="mt-4 flex">
            <div>
                {
                    if user_ctx.is_authenticated() {
                        html! {
                            <img
                                src={ user_ctx.profile_image_url.clone() }
                                class="w-10 h-10 rounded-full"
                                alt={ user_ctx.display_name.clone() }
                            />
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
            <div class="flex-grow ml-2">
                <ShowError error={new_comment.error.clone()} />
                <form onsubmit={onsubmit} class="relative">
                    <textarea
                        class="w-full p-3 border rounded-md resize-none focus:outline-none focus:border-indigo-500"
                        rows="3"
                        placeholder="Write a comment..."
                        value={new_comment_dto.body.clone()}
                        oninput={oninput} >
                    </textarea>
                    <button class="p-1 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 disabled:opacity-50 absolute right-2 bottom-2" type="submit" disabled={button_disabled}>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 12L3.269 3.126A59.768 59.768 0 0121.485 12 59.77 59.77 0 013.27 20.876L5.999 12zm0 0h7.5" />
                        </svg>
                    </button>
                </form>
            </div>
        </div>
    }
}
