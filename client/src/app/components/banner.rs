use yew::prelude::*;

#[function_component(Banner)]
pub fn banner() -> Html {
    html! {
        <div class="bg-gradient-to-r from-blue-500 to-purple-500 text-white">
            <div class="container mx-auto py-8 text-center">
                <h1 class="text-4xl font-semibold">{ "Negatiview" }</h1>
                <p class="mt-4 text-xl">{ "Why You Shouldn't Purchase This Product" }</p>
            </div>
        </div>
    }
}
