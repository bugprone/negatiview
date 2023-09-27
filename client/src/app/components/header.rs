use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::middleware::context::{use_user_context, UserUseStateHandle};
use crate::route::Route;

#[function_component(Header)]
pub fn header() -> Html {
    let user_ctx = use_user_context();
    html! {
        <nav class="bg-white p-4">
            <div class="container mx-auto flex justify-between items-center">
                <div class="text-black text-2xl font-bold">
                    <Link<Route> to={ Route::Home }>{ "Negatiview" }</Link<Route>>
                </div>
                {
                    if user_ctx.is_authenticated() {
                        logged_in_view(&user_ctx)
                    } else {
                        logged_out_view()
                    }
                }
            </div>
        </nav>
    }
}

fn logged_out_view() -> Html {
    html! {
        <ul class="flex space-x-6">
            <li>
                <Link<Route> to={Route::PostList} classes="text-black hover:underline">
                    { "Posts" }
                </Link<Route>>
            </li>
            <li>
                <Link<Route> to={Route::Login} classes="text-black hover:underline">
                    { "Login" }
                </Link<Route>>
            </li>
            <li>
                <Link<Route> to={Route::SignUp} classes="text-black hover:underline">
                    { "Sign Up" }
                </Link<Route>>
            </li>
        </ul>
    }
}

fn logged_in_view(user_ctx: &UserUseStateHandle) -> Html {
    let on_logout = {
        let user_ctx = user_ctx.clone();
        Callback::from(move |_| {
            user_ctx.logout();
        })
    };

    html! {
        <ul class="flex space-x-6">
            <li>
                <Link<Route> to={Route::PostList} classes="text-black hover:underline">
                    { "Posts" }
                </Link<Route>>
            </li>
            <li>
                <button onclick={on_logout} class="text-black hover:underline">
                    { "Logout" }
                </button>
            </li>
            <li>
                <Link<Route> to={Route::Settings} classes="text-black hover:underline font-semibold">
                    { &user_ctx.display_name }
                </Link<Route>>
            </li>
        </ul>
    }
}
