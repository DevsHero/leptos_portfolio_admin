use crate::app::{
    components::{ Experience, RenderTab, Portfolio },
    models::portfolio::{ Experience, Portfolio },
};
use leptos::*;

#[component]
pub fn SelectTab(experiences: Vec<Experience>, portfolios: Vec<Portfolio>) -> impl IntoView {
    let (select_tab, set_select_tab) = create_signal(1);
    let (experiences, set_experiences) = create_signal(experiences);
    let (portfolios, set_portfolios) = create_signal(portfolios);
    view! {
        <section class="tabSection">
            <div class="tabSectionSelector">
                <button
                    class=move || {
                        if select_tab() == 1 { "tabsTitle active" } else { "tabsTitle" }
                    }
                    on:click=move |_| set_select_tab(1)
                >
                   Experiences
                </button>
                <button
                    class=move || {
                        if select_tab() == 2 { "tabsTitle active" } else { "tabsTitle" }
                    }
                    on:click=move |_| set_select_tab(2)
                >
                    Portfolios
                </button>
            </div>
            <RenderTab is_page=false no=1 active_page=select_tab>
            <Experience  
            is_page = false 
            experiences=experiences
            on_delete=None
            use_delete=false
            />    
            </RenderTab>
            <RenderTab is_page=false no=2 active_page=select_tab>
            <Portfolio  is_page = false 
            portfolios=portfolios
            on_delete=None
            use_delete=false
            />  
            </RenderTab>
        </section>
    }
}
