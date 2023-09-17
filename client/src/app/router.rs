use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::health::Healthcheck;
use crate::app::posts::{NewPost, Posts};
use crate::app::users::Users;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/health")]
    Healthcheck,
    #[at("/users")]
    Users,
    #[at("/posts")]
    Posts,
    #[at("/posts/new")]
    NewPost,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <h1> { "Welcome to Negatiview" } </h1> },
        Route::Healthcheck => html! { <Healthcheck /> },
        Route::Users => html! { <Users /> },
        Route::Posts => html! { <Posts /> },
        Route::NewPost => html! { <NewPost /> },
        Route::NotFound => html! { "Page not found" },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
