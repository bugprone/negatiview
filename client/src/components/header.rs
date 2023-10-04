use yew::prelude::*;
use yew_router::prelude::*;

use crate::middlewares::context::{use_user_context, UserUseStateHandle};
use crate::routes::AppRoute;

#[function_component(Header)]
pub fn header() -> Html {
    let user_ctx = use_user_context();
    html! {
        <nav class="bg-white p-4">
            <div class="mx-auto flex justify-between items-center">
                <Link<AppRoute> to={ AppRoute::Home } classes="text-black text-2xl font-bold">
                    { "Negatiview" }
                </Link<AppRoute >>
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
                <Link<AppRoute> to={AppRoute::Home} classes="text-black hover:underline">
                    { "Home" }
                </Link<AppRoute >>
            </li>
            <li>
                <Link<AppRoute> to={AppRoute::Login} classes="text-black hover:underline">
                    { "Login" }
                </Link<AppRoute >>
            </li>
            <li>
                <Link<AppRoute> to={AppRoute::SignUp} classes="text-black hover:underline">
                    { "Sign Up" }
                </Link<AppRoute >>
            </li>
        </ul>
    }
}

fn logged_in_view(user_ctx: &UserUseStateHandle) -> Html {
    html! {
        <ul class="flex space-x-6">
            <li>
                <Link<AppRoute> to={AppRoute::Home} classes="text-black hover:underline">
                    { "Home" }
                </Link<AppRoute >>
            </li>
            <li>
                <Link<AppRoute> to={AppRoute::NewPost} classes="text-black hover:underline">
                    { "New Post" }
                </Link<AppRoute>>
            </li>
            <li>
                <Link<AppRoute> to={AppRoute::Settings} classes="text-black hover:underline">
                    { "Settings" }
                </Link<AppRoute>>
            </li>
            <li>
                <Link<AppRoute> to={AppRoute::Profile { display_name: user_ctx.display_name.clone()}} classes="text-black hover:underline font-semibold">
                    { &user_ctx.display_name }
                </Link<AppRoute >>
            </li>
        </ul>
    }
}
