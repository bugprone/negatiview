use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    #[prop_or_default]
    pub url: Option<String>,
    /// Size and spacing utilities, e.g. `"w-10 h-10 mr-2"`.
    #[prop_or_default]
    pub class: Classes,
}

/// Profile picture, or the user's initial on a tinted disc when there is none.
#[function_component(Avatar)]
pub fn avatar(props: &Props) -> Html {
    let class = classes!("rounded-full", "shrink-0", props.class.clone());
    let url = props.url.as_deref().map(str::trim).filter(|u| !u.is_empty());
    let Some(url) = url else {
        let initial = props
            .name
            .chars()
            .next()
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_default();
        return html! {
            <div
                class={classes!(class, "flex", "items-center", "justify-center", "bg-indigo-200", "text-indigo-700", "font-semibold", "select-none")}
                title={props.name.clone()}
                aria-label={props.name.clone()}
            >
                { initial }
            </div>
        };
    };
    html! { <img src={url.to_owned()} alt={props.name.clone()} class={class} /> }
}
