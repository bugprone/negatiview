use web_sys::Node;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_hooks::prelude::*;

use crate::components::post_meta::PostMeta;
use crate::middlewares::context::use_user_context;
use crate::services::post::get;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub slug: String,
}

#[function_component(Post)]
pub fn post(props: &Props) -> Html {
    let post = {
        let slug = props.slug.clone();
        use_async_with_options(
            async move { get(slug).await },
            UseAsyncOptions::enable_auto(),
        )
    };

    let user_ctx = use_user_context();

    if let Some(resp) = &post.data {
        let post = &resp.data;
        let can_modify =
            user_ctx.is_authenticated() && user_ctx.display_name == post.author.display_name;
        let created_at = post.created_at.format("%B %e %Y").to_string();

        html! {
            <div>
                <div class="bg-indigo-600 py-12 text-white">
                    <div class="mx-auto text-center">
                        <h1 class="text-3xl font-bold">{&post.title}</h1>
                        <PostMeta
                            slug={ post.slug.clone() }
                            author={ post.author.clone() }
                            can_modify={ can_modify }
                            created_at={ created_at }
                        />
                    </div>
                </div>

                <div class="mx-auto py-6 px-4">
                    <div class="row">
                        <div class="col-xs-12">
                            { view_body(&post.body) }
                            <ul class="flex mt-4">
                                {for post.tags.iter().map(|tag| {
                                    html! {
                                        <li class="bg-gray-200 text-gray-600 px-2 py-1 rounded-full mr-2">
                                            { tag }
                                        </li>
                                    }
                                })}
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}

fn view_body(body: &str) -> Html {
    let parser = pulldown_cmark::Parser::new(body);
    let mut html_text = String::new();
    pulldown_cmark::html::push_html(&mut html_text, parser);

    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();

    div.set_inner_html(html_text.as_str());
    let node = Node::from(div);
    VNode::VRef(node)
}
