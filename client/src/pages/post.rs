use web_sys::Node;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_hooks::prelude::*;

use crate::components::comment_list::CommentList;
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
        let can_edit =
            user_ctx.is_authenticated() && user_ctx.display_name == post.author.display_name;
        let created_at = post.created_at.format("%B %e, %Y").to_string();

        html! {
            <div>
                <div class="bg-gradient-to-r from-indigo-800 to-gray-800 text-white py-8">
                    <div class="lg:w-4/5 mx-auto">
                        <h1 class="px-4 mb-4 text-3xl font-bold">{&post.title}</h1>
                        <PostMeta
                            slug={ post.slug.clone() }
                            author={ post.author.clone() }
                            can_edit={ can_edit }
                            created_at={ created_at }
                        />
                    </div>
                </div>
                <div class="lg:w-4/5 mx-auto py-6 px-4">
                    <div class="row">
                        <div class="col-xs-12">
                            { view_body(&post.body) }
                            <ul class="flex mt-4">
                                {for post.tags.iter().map(|tag| {
                                    html! {
                                        <li class="mr-2 inline-flex items-center px-2 py-1 text-sm font-medium text-indigo-400 bg-indigo-100 rounded-full">
                                            { format!("#{}", tag) }
                                        </li>
                                    }
                                })}
                            </ul>
                        </div>
                    </div>
                    <div class="py-6">
                        <h3 class="text-xl font-bold">{ "Comments" }</h3>
                        <CommentList slug={ props.slug.clone() } />
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
