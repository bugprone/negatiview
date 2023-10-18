use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::components::show_error::ShowError;
use crate::routes::AppRoute;
use crate::services::post::{create, get, update};
use crate::types::post::PostUpdateDto;
use crate::types::Wrapper;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub post_id: Option<String>,
}

#[function_component(Editor)]
pub fn editor(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let error = use_state(|| None);
    let update_dto = use_state(PostUpdateDto::default);
    let tag_input = use_state(String::default);

    let post_get = {
        let slug = props.post_id.clone();
        use_async(async move { get(slug.unwrap_or_default()).await })
    };
    let post_update = {
        let slug = props.post_id.clone();
        let update_dto = update_dto.clone();
        use_async(async move {
            let req = Wrapper::<PostUpdateDto> {
                data: (*update_dto).clone(),
            };
            if let Some(slug) = slug {
                update(slug, req).await
            } else {
                create(req).await
            }
        })
    };

    {
        let post_get = post_get.clone();
        use_effect_with(
            props.post_id.clone(),
            move |slug| {
                if slug.is_some() {
                    post_get.run();
                }
                || ()
            },
        );
    }

    {
        let update_dto = update_dto.clone();
        let error = error.clone();
        use_effect_with(
            post_get,
            move |post_get| {
                if let Some(resp) = &post_get.data {
                    update_dto.set(PostUpdateDto {
                        title: resp.data.title.clone(),
                        description: resp.data.description.clone(),
                        body: resp.data.body.clone(),
                        tags: Some(resp.data.tags.clone()),
                    });
                    error.set(None);
                }
                if let Some(err) = &post_get.error {
                    error.set(Some(err.clone()));
                }

                || ()
            },
        );
    }

    {
        let error = error.clone();
        use_effect_with(
            post_update.clone(),
            move |post_update| {
                if let Some(resp) = &post_update.data {
                    error.set(None);
                    navigator.push(&AppRoute::Post {
                        post_id: resp.data.id.clone(),
                    });
                }
                if let Some(err) = &post_update.error {
                    error.set(Some(err.clone()));
                }
                || ()
            },
        );
    }

    let onsubmit = {
        let post_update = post_update.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            post_update.run();
        })
    };

    let oninput_title = {
        let update_dto = update_dto.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut dto = (*update_dto).clone();
            dto.title = input.value();
            update_dto.set(dto);
        })
    };
    let oninput_description = {
        let update_dto = update_dto.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut dto = (*update_dto).clone();
            dto.description = input.value();
            update_dto.set(dto);
        })
    };
    let oninput_body = {
        let update_dto = update_dto.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut dto = (*update_dto).clone();
            dto.body = input.value();
            update_dto.set(dto);
        })
    };
    let oninput_tag = {
        let tag_input = tag_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            tag_input.set(input.value());
        })
    };
    let onkeypress = Callback::from(|e: KeyboardEvent| {
        if e.key() == "Enter" {
            e.prevent_default();
        }
    });
    let onkeyup = {
        let update_dto = update_dto.clone();
        let tag_input = tag_input.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                let mut dto = (*update_dto).clone();
                if let Some(tags) = &mut dto.tags {
                    if !tags.contains(&*tag_input) {
                        tags.push((*tag_input).clone());
                    }
                } else {
                    dto.tags = Some(vec![(*tag_input).clone()]);
                }
                update_dto.set(dto);
                tag_input.set(String::default());
            }
        })
    };

    html! {
        <div class="lg:w-2/3 mx-auto mt-12 mb-12">
            <h1 class="text-center text-xl font-semibold">{ "New Post" }</h1>
            <div class="mt-4">
                <ShowError error={(*error).clone()} />
                <form {onsubmit} class="space-y-4">
                    <div class="form-group">
                        <label for="title" class="block text-sm font-medium text-gray-700">
                            { "Title" }
                        </label>
                        <input
                            class="mt-1 w-full p-2 border rounded"
                            type="text"
                            value={update_dto.title.clone()}
                            oninput={oninput_title}
                        />
                    </div>
                    <div class="form-group">
                        <label for="description" class="block text-sm font-medium text-gray-700">
                            { "Description" }
                        </label>
                        <input
                            class="mt-1 w-full p-2 border rounded"
                            type="text"
                            value={update_dto.description.clone()}
                            oninput={oninput_description}
                        />
                    </div>
                    <div class="form-group">
                        <label for="Content" class="block text-sm font-medium text-gray-700">
                            { "Content" }
                        </label>
                        <textarea
                            class="mt-1 w-full p-2 border rounded resize-none"
                            rows="8"
                            placeholder="You can use markdown text here..."
                            value={update_dto.body.clone()}
                            oninput={oninput_body} >
                        </textarea>
                    </div>
                    <div class="form-group">
                        <label for="tags" class="block text-sm font-medium text-gray-700">
                            { "Tags" }
                        </label>
                        <input
                            class="mt-1 w-full p-2 text-lg border rounded"
                            type="text"
                            value={(*tag_input).clone()}
                            oninput={oninput_tag}
                            {onkeypress}
                            {onkeyup}
                        />
                        <div class="mt-2 flex flex-wrap space-x-2">
                            {
                                if let Some(tags) = &update_dto.tags.clone() {
                                    html! {for tags.iter().map(|tag| {
                                        let onclick_remove = {
                                            let tag = tag.clone();
                                            let update_dto = update_dto.clone();
                                            Callback::from(move |_| {
                                                let mut dto = (*update_dto).clone();
                                                if let Some(tags) = &mut dto.tags {
                                                    tags.retain(|t| t != &tag);
                                                }
                                                update_dto.set(dto);
                                            })
                                        };
                                        html! {
                                            <span class="inline-flex items-center px-2 py-1 text-sm font-medium text-indigo-400 bg-indigo-100 rounded">
                                                { format!("#{}", &tag) }
                                                <button type="button" class="inline-flex items-center p-1 ml-1 text-sm text-indigo-400 bg-transparent rounded-sm hover:bg-indigo-200 hover:text-indigo-600" data-dismiss-target="#badge-dismiss-indigo" aria-label="Remove" onclick={onclick_remove}>
                                                    <svg class="w-2 h-2" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 14">
                                                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"/>
                                                    </svg>
                                                    <span class="sr-only">{"Remove badge"}</span>
                                                </button>
                                            </span>
                                        }
                                    })}
                                } else {
                                    html! {}
                                }
                            }
                        </div>
                    </div>
                    <button
                        class="mt-4 w-full p-2 bg-indigo-600 text-white rounded hover:bg-blue-700 disabled:opacity-50"
                        type="submit"
                        disabled={post_update.loading}>
                        { "Publish" }
                    </button>
                </form>
            </div>
        </div>
    }
}
