use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <nav class="bg-white p-4">
            <div class="container mx-auto">
                <a class="text-black text-2xl font-bold" href="/">{ "Negatiview" }</a>
            </div>
        </nav>
    }
}
