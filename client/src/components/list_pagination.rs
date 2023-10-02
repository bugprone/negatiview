use yew::prelude::*;

const ITEMS_PER_PAGE: u32 = 10;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub total: u32,
    pub current_page: u32,
    pub callback: Callback<u32>,
}

#[function_component(ListPagination)]
pub fn list_pagination(props: &Props) -> Html {
    if props.total < ITEMS_PER_PAGE {
        return html! {};
    }

    let max_page = (props.total as f32 / 10.0).ceil() as u32;
    let mut pages: Vec<u32> = vec![];
    for page in 0..max_page {
        pages.push(page);
    }

    html! {
        <nav>
            <ul class="pagination">
            {for pages.iter().map(|page| {
                let is_current = page == &props.current_page;
                let page_item_class = if is_current {
                    "pages-item active"
                } else {
                    "pages-item"
                };
                let page = *page;
                let callback = props.callback.clone();
                let onclick = Callback::from(move |ev: MouseEvent| {
                    ev.prevent_default();
                    callback.emit(page)
                });
                html! {
                    <li
                        class={page_item_class}
                        onclick={onclick}>
                        <a class="pages-link" href="">{page + 1}</a>
                    </li>
                }
            })}
            </ul>
        </nav>
    }
}
