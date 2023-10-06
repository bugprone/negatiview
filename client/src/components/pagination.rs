use yew::prelude::*;

const ITEMS_PER_PAGE: usize = 10;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub total: usize,
    pub current_page: usize,
    pub callback: Callback<usize>,
}

#[function_component(Pagination)]
pub fn list_pagination(props: &Props) -> Html {
    if props.total < ITEMS_PER_PAGE {
        return html! {};
    }

    let max_page = (props.total as f32 / ITEMS_PER_PAGE as f32).ceil() as usize;
    let mut pages: Vec<usize> = vec![];
    for page in 0..max_page {
        pages.push(page);
    }

    html! {
        <nav class="flex justify-center">
            <ul class="inline-flex -space-x-px text-sm">
            {for pages.iter().map(|page| {
                let is_current = page == &props.current_page;
                let page_item_class = if is_current {
                    "flex items-center justify-center px-3 h-8 text-blue-600 border border-gray-300 bg-blue-50 hover:bg-blue-100 hover:text-blue-700"
                } else {
                    "flex items-center justify-center px-3 h-8 leading-tight text-gray-500 bg-white border border-gray-300 hover:bg-gray-100 hover:text-gray-700"
                };
                let page = *page;
                let callback = props.callback.clone();
                let onclick = Callback::from(move |ev: MouseEvent| {
                    ev.prevent_default();
                    callback.emit(page)
                });
                html! {
                    <li
                        onclick={onclick}>
                        <a href="" class={page_item_class}>{page + 1}</a>
                    </li>
                }
            })}
            </ul>
        </nav>
    }
}
