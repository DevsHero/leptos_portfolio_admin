use crate::app::{
    components::{ Experience, RenderTab, Portfolio },
    models::portfolio::{ Experience, Portfolio },
};
use leptos::*;

#[component]
pub fn SelectTab(
    experiences: Vec<Experience>,
    portfolios: Vec<Portfolio>,
    is_ready: ReadSignal<bool>
) -> impl IntoView {
    let (select_tab, set_select_tab) = create_signal(1);
    let (experiences, _set_experiences) = create_signal(experiences);
    let (portfolios, _set_portfolios) = create_signal(portfolios);
    let count_experience = experiences.get().len();
    let count_portfolio = portfolios.get().len();
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
                 <span  class=move || if !is_ready.get() { "loadingTab " } else { "tabRowBadget" } >  Experiences {if count_experience > 0  {Some(view! {<p class="badget">  {count_experience}</p>})} else {None}} </span> 
                </button>
                <button
                type="button" 
                    class=move || {
                        if select_tab() == 2 { "tabsTitle active" } else { "tabsTitle" }
                    }
                    on:click=move |_| set_select_tab(2)
                > 
                  <span  class=move || if !is_ready.get() { "loadingTab " } else { "tabRowBadget" }>  Portfolios {if count_portfolio > 0  {Some(view! {<p class="badget">  {count_portfolio}</p>})} else {None}} </span> 
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
