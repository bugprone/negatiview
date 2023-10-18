use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::post_action::PostAction;
use crate::routes::AppRoute;
use crate::types::profile::ProfileDto;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub post_id: String,
    pub can_edit: bool,
    pub author: ProfileDto,
    pub created_at: String,
}

#[function_component(PostMeta)]
pub fn post_meta(props: &Props) -> Html {
    html! {
        <div class="px-4 flex justify-between">
            <div class="flex">
                <img src={props.author.profile_image_url.clone()} alt={props.author.display_name.clone()} class="w-12 h-12 rounded-full mr-2" />
                <div>
                    <div class="text-white text-left hover:underline">
                        <Link<AppRoute> to={AppRoute::Profile { display_name: props.author.display_name.clone() }}>
                            { &props.author.display_name }
                        </Link<AppRoute >>
                    </div>
                    <span class="text-gray-500 text-sm">
                        { &props.created_at }
                    </span>
                </div>
            </div>

            <PostAction post_id={props.post_id.clone()} can_edit={props.can_edit} />
        </div>
    }
}
