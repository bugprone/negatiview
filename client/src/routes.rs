use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{
    footer::Footer,
    header::Header,
    user_context_provider::UserContextProvider,
};
use crate::pages::{
    health::Healthcheck,
    home::Home,
    login::Login,
    post::Post,
    profile::{Profile, ProfileTab},
    settings::Settings,
    sign_up::SignUp,
    users::Users
};
use crate::pages::editor::Editor;

#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
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
    #[at("/settings")]
    Settings,
    #[at("/editor")]
    NewPost,
    #[at("/editor/:slug")]
    EditPost { slug: String },
    #[at("/posts/:slug")]
    Post { slug: String },
    #[at("/profile/:display_name")]
    Profile { display_name: String },
    #[at("/profile/:display_name/follow")]
    Follow { display_name: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! { <Home /> },
        AppRoute::Healthcheck => html! { <Healthcheck /> },
        AppRoute::Users => html! { <Users /> },
        AppRoute::SignUp => html! { <SignUp /> },
        AppRoute::Login => html! { <Login /> },
        AppRoute::Settings => html! { <Settings /> },
        AppRoute::NewPost => html! { <Editor /> },
        AppRoute::EditPost { slug } => html! { <Editor slug={slug} /> },
        AppRoute::Post { slug } => html! { <Post slug={slug} /> },
        AppRoute::Profile { display_name } => html! {
            <Profile display_name={display_name} tab={ProfileTab::ByAuthor} />
        },
        AppRoute::Follow { display_name } => html! {
            <Profile display_name={display_name} tab={ProfileTab::FollowedBy} />
        },
        AppRoute::NotFound => html! { <p class="text-red-500">{ "Page not found" }</p> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <UserContextProvider>
                <Header />
                <Switch<AppRoute> render={switch} />
                <Footer />
            </UserContextProvider>
        </BrowserRouter>
    }
}
