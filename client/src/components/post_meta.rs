use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;
use crate::types::profile::Profile;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub slug: String,
    pub can_modify: bool,
    pub author: Profile,
    pub created_at: String,
}

#[function_component(PostMeta)]
pub fn post_meta(props: &Props) -> Html {
    html! {
        <div class="flex items-center mb-4">
            <img src={props.author.profile_image_url.clone()} alt={props.author.display_name.clone()} class="w-8 h-8 rounded-full mr-2" />
            <div class="info">
                <div class="text-indigo-600 hover:underline">
                    <Link<AppRoute> to={AppRoute::Profile { display_name: props.author.display_name.clone() }}>
                        {&props.author.display_name}
                    </Link<AppRoute >>
                </div>
                <span class="date text-gray-500">
                    {&props.created_at}
                </span>
            </div>
            // <ArticleActions can_modify={props.can_modify} slug={props.slug.clone()} />
        </div>
    }
}
