use leptos::*;
use leptos::{ component, view, IntoView };
#[component]
pub fn RenderTab(
    no: i32,
    active_page: ReadSignal<i32>,
    children: Children,
    is_page: bool
) -> impl IntoView {
    let class = Memo::new(move |_| {
        if !is_page {
            if no == active_page() { "tabContainer activePage" } else { "tabContainer" }
        } else {
            if no == active_page() { "tabPageContainer activePage" } else { "tabPageContainer" }
        }
    });
    view! {
        <div class=class id=no>
            {children()}
       
        </div>
        }
}
