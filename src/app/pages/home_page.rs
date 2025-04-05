use crate::app::{
    components::{ Dialog, HomeContacts, LanguageChips, Loading, SelectTab, SkillChips },
    server::api::get_profile,
    utils::utils::calculate_age,
};
use leptos::*;
use std::time::Duration;
use crate::app::utils::utils::{ getLocalStorage, setLocalStorage };
use chrono::{ DateTime, Utc };
#[component]
pub fn HomePage() -> impl IntoView {
    let get_profile_info = Resource::new(
        || (),
        move |_| async move { get_profile().await }
    );
    let (is_ready, set_is_ready) = create_signal(false);
    let (timer_finished, set_timer_finished) = create_signal(false);
    const LOCAL_STORAGE_VISIT_KEY: &str = "last_visit_time_ms";

    //This effect is used to set a loading intro timeout and display it once every 24 hours based on the time of the last visit.
    create_effect(move |_| {
        let now_dt: DateTime<Utc> = Utc::now();
        let now_ms = now_dt.timestamp_millis();
        let twenty_four_hours_ms = 24 * 60 * 60 * 1000;
        let mut delay_seconds = 7;
        let visit_time_js_value = getLocalStorage(LOCAL_STORAGE_VISIT_KEY);
        if visit_time_js_value.is_null() || visit_time_js_value.is_undefined() {
        } else {
            match visit_time_js_value.as_string() {
                Some(visit_time_str) => {
                    match visit_time_str.parse::<i64>() {
                        Ok(last_visit_ms) => {
                            let time_since_last_visit_ms = now_ms.saturating_sub(last_visit_ms);
                            leptos::logging::log!(
                                "Last visit was {} ms ago ({} hours)",
                                time_since_last_visit_ms,
                                time_since_last_visit_ms / (60 * 60 * 1000)
                            );
                            if time_since_last_visit_ms < twenty_four_hours_ms {
                                delay_seconds = 0;
                            } else {
                            }
                        }
                        Err(_) => {}
                    }
                }
                None => {}
            }
        }

        let handle = set_timeout(move || {
            set_timer_finished.set(true);
        }, Duration::from_secs(delay_seconds));

        on_cleanup(move || {
            let _ = handle;
        });
        // --- Update the last visit time in localStorage ---

        setLocalStorage(LOCAL_STORAGE_VISIT_KEY, &now_ms.to_string());
    });

    create_effect(move |_| {
        set_is_ready.set(true);
    });
    view! {
        
        <Suspense fallback=Loading>
            {move || { 
                match get_profile_info.get() {
                    Some(Ok(profile)) => {
                        if timer_finished.get() {
                        let (skills, _) = create_signal(profile.skills.clone().unwrap_or_default());
                        let (languages, _) = create_signal(profile.languages.clone().unwrap_or_default());
                        let (birth_date, set_birth_date) = create_signal(String::new());
                        let (open_dialog, set_open_dialog) = create_signal(false);
                        let avatar =  profile.avatar.clone();
                        let profile_clone = profile.clone();
                        create_effect(move |_| {
                        let age = calculate_age(&profile.birth_date);
                            set_birth_date.set(age.to_string());
                        });
                        view! {
                            <div class="indexLayout">
                            { move || { 
                             if open_dialog.get() { 
                                 let clone_avatar =  profile.avatar.clone();
                                 view!  {<div  on:click=move |_| {
                                 set_open_dialog.set(!open_dialog.get()); }>
                                 <Dialog children_only=true >
                                 <img alt="avatar" src={clone_avatar.clone()} />
                             </Dialog>
                                 </div>}}
                             else {
                                 view! {<div></div>}
                             } 
                            }  }
                              <div></div>
                                 <section class="info">
                                     <div class="profile">
                                         <span class="avatar">
                                                 <button type="button" class="avatar" 
                                                 on:click=move |_| {
                                                     set_open_dialog.set(!open_dialog.get());   } >
                                                 <img alt="avatar" src={avatar.clone()}  />
                                                 </button>                            
                                             <div class="details">
                                             <h1>{profile.first_name.clone()}" "{profile.last_name.clone()}</h1>
                                             <p><b>Nick Name: </b>{profile.nick_name.clone()}</p>
                                                 <p><b>Job Title: </b>{profile.role.clone()}</p>
                                                 <div class="info-row">
                                                     <p><b>Age: </b> {birth_date}</p>
                                                     <p><b>Gender: </b>{profile.gender.clone()}</p>
                                                 </div>
                                                 <p><b>Nationality: </b>{profile.nationality.clone()}</p>
                                                 <p><b>Address: </b>{profile.address.clone()}</p>
                                             </div>
                                         </span>
                                         <HomeContacts profile=profile_clone.clone()   is_ready=is_ready   />
                                     </div>
                                     <div class="about">
                                         <h2>About Me</h2>
                                       <div class="aboutDetail" inner_html={profile.about.clone() }>  </div>
                                     </div>
                                     <div class="skills">
                                         <h2>Skills</h2>
                                         <div>
                                             <SkillChips
                                                 skills=skills
                                                 is_edit=false
                                             />
                                         </div>
                                     </div>
                                     <div class="skills">
                                     <h2>Languages</h2>
                                     <div>
                                         <LanguageChips 
                                             languages=languages
                                             is_edit=false
                                         />
                                     </div>
                                 </div>
                                 </section>
                                 <SelectTab 
                                 is_ready=is_ready
                                     experiences={profile.experiences.clone().unwrap_or_default()} 
                                     portfolios={profile.portfolios.clone().unwrap_or_default()}
                                     educations={profile.educations.clone().unwrap_or_default()}
                                 />
                             </div>
                                                 }
                        }
                        else {
                            // Data loaded, but timer not finished -> show loading
                             leptos::logging::log!("Profile loaded, waiting for timer...");
                            view! { <div><Loading /></div> } 
                        } },
                    Some(Err(e)) => view! { 
                        <div class="indexLayout">
                            <div>"Error loading profile: "{e.to_string()}</div>
                        </div> 
                    },
                    None => view! { 
                        <div class="indexLayout">
                            <div>"Loading..."</div>
                        </div> 
                    }
                }
            }}
        </Suspense>
    }
}
