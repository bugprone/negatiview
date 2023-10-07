use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;
use crate::services::post::*;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub slug: String,
    pub can_edit: bool,
}

#[function_component(PostAction)]
pub fn post_action(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let delete = {
        let slug = props.slug.clone();
        use_async(async move { del(slug).await})
    };

    let onclick = {
        let delete = delete.clone();
        Callback::from(move |_| {
            delete.run()
        })
    };

    use_effect_with(
        delete,
        move |delete| {
            if delete.data.is_some() {
                navigator.push(&AppRoute::Home);
            }
            || ()
        }
    );

    if props.can_edit {
        html! {
            <span class="flex space-x-2 items-center text-center">
                <Link<AppRoute> to={AppRoute::EditPost { slug: props.slug.clone() }} classes="flex items-center border-2 px-2 py-2 rounded w-24 justify-center hover:bg-blue-500">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5" >
                        <path d="M5.433 13.917l1.262-3.155A4 4 0 017.58 9.42l6.92-6.918a2.121 2.121 0 013 3l-6.92 6.918c-.383.383-.84.685-1.343.886l-3.154 1.262a.5.5 0 01-.65-.65z" />
                        <path d="M3.5 5.75c0-.69.56-1.25 1.25-1.25H10A.75.75 0 0010 3H4.75A2.75 2.75 0 002 5.75v9.5A2.75 2.75 0 004.75 18h9.5A2.75 2.75 0 0017 15.25V10a.75.75 0 00-1.5 0v5.25c0 .69-.56 1.25-1.25 1.25h-9.5c-.69 0-1.25-.56-1.25-1.25v-9.5z" />
                    </svg>
                    <span class="text-sm ml-1">
                        { "Modify" }
                    </span>
                </Link<AppRoute>>
                <button onclick={onclick} class="flex items-center border-2 px-2 py-2 rounded w-24 justify-center hover:bg-red-500">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
                        <path fill-rule="evenodd" d="M8.75 1A2.75 2.75 0 006 3.75v.443c-.795.077-1.584.176-2.365.298a.75.75 0 10.23 1.482l.149-.022.841 10.518A2.75 2.75 0 007.596 19h4.807a2.75 2.75 0 002.742-2.53l.841-10.52.149.023a.75.75 0 00.23-1.482A41.03 41.03 0 0014 4.193V3.75A2.75 2.75 0 0011.25 1h-2.5zM10 4c.84 0 1.673.025 2.5.075V3.75c0-.69-.56-1.25-1.25-1.25h-2.5c-.69 0-1.25.56-1.25 1.25v.325C8.327 4.025 9.16 4 10 4zM8.58 7.72a.75.75 0 00-1.5.06l.3 7.5a.75.75 0 101.5-.06l-.3-7.5zm4.34.06a.75.75 0 10-1.5-.06l-.3 7.5a.75.75 0 101.5.06l.3-7.5z" clip-rule="evenodd" />
                    </svg>
                    <span class="text-sm ml-1">
                        { "Delete" }
                    </span>
                </button>
            </span>
        }
    } else {
        html! {
            <span>
            </span>
        }
    }
}
