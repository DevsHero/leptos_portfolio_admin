use crate::app::{
    components::{ Experience, RenderTab, Portfolio },
    models::portfolio::{ Experience, Portfolio },
};
use leptos::*;

#[component]
pub fn SelectTab(experiences: Vec<Experience>, portfolios: Vec<Portfolio>) -> impl IntoView {
    let (select_tab, set_select_tab) = create_signal(1);
    let (experiences, _set_experiences) = create_signal(experiences);
    let (portfolios, _set_portfolios) = create_signal(portfolios);
    view! {
        <section class="tabSection">
            <div class="tabSectionSelector">
                <button
                type="button" 
                    class=move || {
                        if select_tab() == 1 { "tabsTitle active" } else { "tabsTitle" }
                    }
                    on:click=move |_| set_select_tab(1)
                >
                 <span class="tabRowBadget">  Experiences <p class="badget"> {experiences.get().len()} </p> </span> 
                </button>
                <button
                type="button" 
                    class=move || {
                        if select_tab() == 2 { "tabsTitle active" } else { "tabsTitle" }
                    }
                    on:click=move |_| set_select_tab(2)
                >
                <span class="tabRowBadget">  Portfolios <p class="badget"> {portfolios.get().len()} </p> </span>    
                </button>
            </div>
            <RenderTab  no=1 active_page=select_tab>
            <Experience   
            experiences=experiences
           
            is_edit=false
            />    
            </RenderTab>
            <RenderTab  no=2 active_page=select_tab>
            
            <Show when=move || select_tab() == 2>
            // Only render when first activated
            <Suspense fallback=move || view! { <p>"Loading ..."</p> }>
            <Portfolio  
            portfolios=portfolios
         
            is_edit=false
            />  
            </Suspense>
        </Show>
          
            </RenderTab>
        </section>
    }
}
