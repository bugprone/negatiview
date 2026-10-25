use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

#[function_component(Footer)]
pub fn footer() -> Html {
    let year = web_sys::js_sys::Date::new_0().get_full_year();
    html! {
        <div class="container mx-auto">
            <footer class="bg-white py-4 text-center">
                <span class="text-gray-600 text-sm">
                    { format!(" bugprone • © 2023–{year} • ") }
                    <Link<AppRoute> to={ AppRoute::Home }>{ "Negatiview" }</Link<AppRoute>>
                </span>
            </footer>
        </div>
    }
}
