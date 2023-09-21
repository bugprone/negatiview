use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::components::footer::Footer;
use crate::app::components::header::Header;
use crate::app::components::login::Login;
use crate::app::components::new_post::NewPost;
use crate::app::components::post_list::PostList;
use crate::app::components::sign_up::SignUp;
use crate::app::components::user_context_provider::UserContextProvider;
use crate::app::health::Healthcheck;
use crate::app::users::Users;
use crate::app::home::Home;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/health")]
    Healthcheck,
    #[at("/users")]
    Users,
    #[at("/users/sign_up")]
    SignUp,
    #[at("/users/login")]
    Login,
    #[at("/posts")]
    PostList,
    #[at("/posts/new")]
    NewPost,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Healthcheck => html! { <Healthcheck /> },
        Route::Users => html! { <Users /> },
        Route::SignUp => html! { <SignUp /> },
        Route::Login => html! { <Login /> },
        Route::PostList => html! { <PostList /> },
        Route::NewPost => html! { <NewPost /> },
        Route::NotFound => html! { <p class="text-red-500">{ "Page not found" }</p> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <UserContextProvider>
                <Header />
                <Switch<Route> render={switch} />
                <Footer />
            </UserContextProvider>
        </HashRouter>
    }
}
