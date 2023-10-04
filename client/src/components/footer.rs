use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class="container mx-auto">
            <footer class="bg-white py-4 text-center">
                <span class="text-gray-600 text-sm">
                    {" bugprone • © 2023 • "}
                    <Link<AppRoute> to={ AppRoute::Home }>{ "Negatiview" }</Link<AppRoute>>
                </span>
            </footer>
        </div>
    }
}
