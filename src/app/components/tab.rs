use crate::app::{ components::{ Experience, RenderTab }, models::portfolio::Experience };
use leptos::*;

#[component]
pub fn SelectTab(experiences: Vec<Experience>) -> impl IntoView {
    let (select_tab, set_select_tab) = create_signal(1);
    view! {
        <section class="tabSection">
            <div class="tabSectionSelector">
                <button
                    class=move || {
                        if select_tab() == 1 { "tabsTitle active" } else { "tabsTitle" }
                    }
                    on:click=move |_| set_select_tab(1)
                >
                   Experience
                </button>
                <button
                    class=move || {
                        if select_tab() == 2 { "tabsTitle active" } else { "tabsTitle" }
                    }
                    on:click=move |_| set_select_tab(2)
                >
                    My Project
                </button>
            </div>
            <RenderTab is_page=false no=1 active_page=select_tab>
            { experiences.into_iter().enumerate().map(|(index, experience)| {
                view! {
                    <Experience is_page=false experience=experience index=(index + 1).to_string()/>
                }
            }).collect::<Vec<_>>() }
            </RenderTab>
            <RenderTab is_page=false no=2 active_page=select_tab>
            <p>"RenderTab 2"</p>
            </RenderTab>
        </section>
    }
}
