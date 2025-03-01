use crate::app::{ components::Experience, models::portfolio::Experience };
use leptos::*;
#[component]
pub fn Tab(no: i32, active_page: ReadSignal<i32>, children: Children) -> impl IntoView {
    let class = Memo::new(move |_| {
        if no == active_page() { "tabContainer activePage" } else { "tabContainer" }
    });
    view! {
        <div class=class id=no>
            {children()}
        </div>
    }
}
#[component]
pub fn SelectTab(experiences: Vec<Experience>) -> impl IntoView {
    let (page, set_page) = create_signal(1);
    view! {
        <section class="projectSection">
            <div class="projectSectionSelector">
                <button
                    class=move || {
                        if page() == 1 { "projectsTitle active" } else { "projectsTitle" }
                    }
                    on:click=move |_| set_page(1)
                >
                   Experience
                </button>
                <button
                    class=move || {
                        if page() == 2 { "projectsTitle active" } else { "projectsTitle" }
                    }
                    on:click=move |_| set_page(2)
                >
                    My Project
                </button>
            </div>
            <Tab no=1 active_page=page>
            { experiences.into_iter().enumerate().map(|(index, experience)| {
                view! {
                    <Experience  experience=experience index=(index + 1).to_string()/>
                }
            }).collect::<Vec<_>>() }
            </Tab>
            <Tab no=2 active_page=page>
            <p>"Tab 2"</p>
            </Tab>
        </section>
    }
}
