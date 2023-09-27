use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="bg-gray-200 py-4">
            <div class="container mx-auto text-center">
                <span class="text-gray-600">
                    {" bugprone • © 2023 • "}
                    <Link<Route> to={ Route::Home }>{ "Negatiview" }</Link<Route>>
                </span>
            </div>
        </footer>
    }
}
