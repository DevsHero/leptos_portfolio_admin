use crate::app::{
    components::{
        layouts::TabRender,
        records::{ EducationRecords, ExperienceRecords, PortfolioRecords },
    },
    models::profile::{ Education, Experience, Portfolio },
};
use leptos::*;

#[component]
pub fn HomeTabs(
    experiences: Vec<Experience>,
    portfolios: Vec<Portfolio>,
    educations: Vec<Education>,
    is_ready: ReadSignal<bool>
) -> impl IntoView {
    let (select_tab, set_select_tab) = create_signal(1);
    let (experiences, _set_experiences) = create_signal(experiences);
    let (portfolios, _set_portfolios) = create_signal(portfolios);
    let (educations, _set_educations) = create_signal(educations);
    let count_experience = experiences.get().len();
    let count_portfolio = portfolios.get().len();
    let count_education = educations.get().len();
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
                <button
                type="button" 
                    class=move || {
                        if select_tab() == 3 { "tabsTitle active" } else { "tabsTitle" }
                    }
                    on:click=move |_| set_select_tab(3)
                > 
                  <span  class=move || if !is_ready.get() { "loadingTab " } else { "tabRowBadget" }>  Education {if count_education > 0  {Some(view! {<p class="badget">  {count_education}</p>})} else {None}} </span> 
                </button>
            </div>
            <TabRender  no=1 active_page=select_tab>
            <ExperienceRecords
            experiences=experiences 
            is_edit=false
            />    
            </TabRender>
            <TabRender  no=2 active_page=select_tab>    
            <Show when=move || select_tab() == 2>
            <Suspense fallback=move || view! { <p>"LoadingIntro ..."</p> }>
            <PortfolioRecords
            portfolios=portfolios
            is_edit=false
            />  
            </Suspense>
            </Show>
            </TabRender>
            <TabRender no=3 active_page=select_tab>    
            <Show when=move || select_tab() == 3>
            <Suspense fallback=move || view! { <p>"LoadingIntro ..."</p> }>
            <EducationRecords
            educations=educations
            is_edit=false
            />  
            </Suspense>
            </Show>
            </TabRender>
        </section>
    }
}
