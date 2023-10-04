use yew::prelude::*;

#[function_component(Banner)]
pub fn banner() -> Html {
    html! {
        <div class="mx-auto flex justify-between items-center bg-gradient-to-r from-indigo-800 to-gray-800 text-white">
            <div class="mx-auto py-12 text-center">
                <h1 class="text-4xl font-extrabold tracking-tight">{ "Negatiview" }</h1>
                <p class="mt-4 text-xl font-semibold">{ "Why You Shouldn't Purchase This Product" }</p>
            </div>
        </div>
    }
}
