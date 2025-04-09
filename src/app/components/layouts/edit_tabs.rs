use leptos::*;
use leptos_icons::Icon;
use icondata as i;

use crate::app::models::{ Experience, Skill, Portfolio, Contact, Language, Education };
#[component]
pub fn EditMenu(
    set_select_tab: WriteSignal<i32>,
    select_tab: ReadSignal<i32>,
    experiences: ReadSignal<Vec<Experience>>,
    skills: ReadSignal<Vec<Skill>>,
    portfolios: ReadSignal<Vec<Portfolio>>,
    contacts: ReadSignal<Vec<Contact>>,
    languages: ReadSignal<Vec<Language>>,
    educations: ReadSignal<Vec<Education>>
) -> impl IntoView {
    let (is_mobile, set_is_mobile) = create_signal(false);
    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(width) = window.inner_width().map(|w| w.as_f64().unwrap_or(0.0)) {
                set_is_mobile(width < 768.0);
            }
        }
    });

    view! {
        <div class="tabSectionSelector" >
                      <button
                      type="button"
                          class=move || {
                              if select_tab() == 1 { "tabsTitle active" } else { "tabsTitle" }
                          }
                          on:click=move |_| set_select_tab(1)  >
                          {move || if is_mobile.get() {view! { <span class="editTabIcon" >    <Icon icon={i::CgProfile} />  </span>} } else { view! {<span class="editTabRowBadget"> Profile  </span>} }}
                         
                      </button>
                      <button
                      type="button"
                          class=move || {
                              if select_tab() == 2 { "tabsTitle active" } else { "tabsTitle" }
                          }
                          on:click=move |_| set_select_tab(2)   >   
                          <span class="editTabRowBadget">    {move || if is_mobile.get() {view! { <p class="editTabIcon" >    <Icon icon={i::MdiLightbulbOn60} />  </p>} } else { view! {  <p> Skill </p>   } }}
                          {move || if skills.get().len() > 0 {view! {<p class="badget">  {skills.get().len()}</p>}}else{view! {<p></p>}}}  </span>
                      </button>
                      <button
                      type="button"
                       class=move || {
                          if select_tab() == 3 { "tabsTitle active" } else { "tabsTitle" }
                      }
                      on:click=move |_| set_select_tab(3)  >
                      <span class="editTabRowBadget">    {move || if is_mobile.get() {view! { <p class="editTabIcon" >    <Icon icon={i::BsSuitcaseLgFill} />  </p>} } else { view! {  <p> Experience </p>   } }}
                      {move || if experiences.get().len() > 0 {view! {<p class="badget">  {experiences.get().len()}</p>}}else{view! {<p></p>}}}  </span>
                  </button>
                  <button
                  type="button"
                  class=move || {
                      if select_tab() == 4 { "tabsTitle active" } else { "tabsTitle" }
                  }
                  on:click=move |_| set_select_tab(4) >
                  <span class="editTabRowBadget">    {move || if is_mobile.get() {view! { <p class="editTabIcon" >    <Icon icon={i::BiPhotoAlbumRegular} />  </p>} } else { view! {  <p> Portfolio </p>   } }}
                  {move || if portfolios.get().len() > 0 {view! {<p class="badget">  {portfolios.get().len()}</p>}}else{view! {<p></p>}}}  </span>
               
                  </button>
                <button
                type="button"
                class=move || {
                  if select_tab() == 5 { "tabsTitle active" } else { "tabsTitle" }
                }
                on:click=move |_| set_select_tab(5)  >
                <span class="editTabRowBadget">    {move || if is_mobile.get() {view! { <p class="editTabIcon" >    <Icon icon={i::MdiPhoneLogOutline} />  </p>} } else { view! {  <p> Contact </p>   } }}
                {move || if contacts.get().len() > 0 {view! {<p class="badget">  {contacts.get().len()}</p>}}else{view! {<p></p>}}}  </span>
             
                </button>
                <button
                type="button"
                class=move || {
                  if select_tab() == 6 { "tabsTitle active" } else { "tabsTitle" }
                }
                on:click=move |_| set_select_tab(6)  >
                <span class="editTabRowBadget">    {move || if is_mobile.get() {view! { <p class="editTabIcon" >    <Icon icon={i::IoSchoolSharp} />  </p>} } else { view! {  <p> Education </p>   } }}
                {move || if educations.get().len() > 0 {view! {<p class="badget">  {educations.get().len()}</p>}}else{view! {<p></p>}}}  </span>
              </button>
                <button
                type="button"
                class=move || {
                  if select_tab() == 7 { "tabsTitle active" } else { "tabsTitle" }
                }
                on:click=move |_| set_select_tab(7)  >
                <span class="editTabRowBadget">    {move || if is_mobile.get() {view! { <p class="editTabIcon" >    <Icon icon={i::TbLanguage} />  </p>} } else { view! {  <p> Language </p>   } }}
                {move || if languages.get().len() > 0 {view! {<p class="badget">  {languages.get().len()}</p>}}else{view! {<p></p>}}}  </span>
               </button>
               <button
                      type="button"
                          class=move || {
                              if select_tab() ==8 { "tabsTitle active" } else { "tabsTitle" }
                          }
                          on:click=move |_| set_select_tab(8)  >
                          {move || if is_mobile.get() {view! { <span class="editTabIcon" >    <Icon icon={i::BsFiletypePdf} />  </span>} } else { view! {<span class="editTabRowBadget"> PDF  </span>} }}
                         
                      </button>
                  </div>
    }
}
