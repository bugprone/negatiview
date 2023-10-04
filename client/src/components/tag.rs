use yew::prelude::*;
use yew_hooks::prelude::*;
use crate::services::tag::get;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub callback: Callback<String>,
}

#[function_component(Tags)]
pub fn tags(props: &Props) -> Html {
    let tags = use_async_with_options(
        async move { get().await },
        UseAsyncOptions::enable_auto(),
    );

    if let Some(resp) = &tags.data {
        html! {
            <div class="container mx-auto flex flex-wrap">
                { for resp.data.tags.iter().map(|tag| {
                    let onclick = {
                        let tag = tag.clone();
                        let callback = props.callback.clone();
                        Callback::from(move |e: MouseEvent| {
                            e.prevent_default();
                            callback.emit(tag.clone());
                        })
                    };
                    html! {
                        <a
                            href=""
                            class="mr-2 mb-2 px-2 py-1 text-sm font-medium text-indigo-400 bg-indigo-100 rounded-full hover:bg-indigo-200"
                            onclick={onclick}
                        >
                            { format!("#{}", &tag) }
                        </a>
                    }
                })}
            </div>
        }
    } else {
        html! {
            <div>{ "Loading" }</div>
        }
    }
}
