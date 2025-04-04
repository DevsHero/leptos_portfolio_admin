use leptos::*;
#[component]
pub fn RenderTab(no: i32, active_page: ReadSignal<i32>, children: Children) -> impl IntoView {
    let class = Memo::new(move |_| {
        if no == active_page() { "tabContainer activePage" } else { "tabContainer" }
    });
    view! {
        <div class=class id=no>
            {children()}
        </div>
        }
}
