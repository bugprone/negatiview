use yew::prelude::*;

#[function_component(Banner)]
pub fn banner() -> Html {
    html! {
        <div class="bg-gradient-to-r from-indigo-800 to-gray-800 text-white">
            <div class="container mx-auto py-8 text-center">
                <h1 class="text-4xl font-extrabold tracking-tight">{ "Negatiview" }</h1>
                <p class="mt-4 text-xl font-semibold">{ "Why You Shouldn't Purchase This Product" }</p>
            </div>
        </div>
    }
}
