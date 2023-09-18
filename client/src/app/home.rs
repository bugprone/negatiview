use yew::prelude::*;

use crate::app::components::post_list::PostList;
use crate::app::components::banner::Banner;

#[function_component(MainPage)]
pub fn main_page() -> Html {
    html! {
        <div class="col-span-9">
            <div class="grid grid-cols-12 gap-4">
                <div class="col-span-12 sm:col-span-12 md:col-span-9">
                    <div class="posts">
                        <ul class="flex flex-wrap space-x-2">
                        </ul>
                    </div>
                </div>
            </div>
            <PostList />
        </div>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <Banner />
            <div class="container mx-auto py-8">
                <div class="grid grid-cols-12 gap-4">
                    <MainPage />
                    <div class="col-span-12 sm:col-span-12 md:col-span-3">
                        <div class="sidebar">
                            <p> { "Sidebar" } </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
