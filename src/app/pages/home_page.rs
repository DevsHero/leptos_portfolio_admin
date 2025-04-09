use crate::app::{
    components::{
        layouts::{ Dialog, HomeTabs, Topbar },
        records::{ ContactRecords, LanguageRecords, SkillRecords },
        utils::LoadingIntro,
    },
    server::api::get_profile_api,
    utils::utils::calculate_age,
};
use leptos::*;
use std::time::Duration;
#[component]
pub fn HomePage() -> impl IntoView {
    let get_profile_api_info = Resource::new(
        || (),
        move |_| async move { get_profile_api().await }
    );
    let (is_ready, set_is_ready) = create_signal(false);
    let (timer_finished, set_timer_finished) = create_signal(false);

    //This effect is used to set a loading intro timeout
    create_effect(move |_| {
        let handle = set_timeout(move || {
            set_timer_finished.set(true);
        }, Duration::from_secs(3));

        on_cleanup(move || {
            let _ = handle;
        });
    });

    create_effect(move |_| {
        set_is_ready.set(true);
    });
    view! {
        
        <Suspense fallback=LoadingIntro>
            {move || { 
                match get_profile_api_info.get() {
                    Some(Ok(profile)) => {
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
                       
                        <div>
                        { move || { if timer_finished.get() {view! { <div> </div> } }
                        else {    
                            view! { <div><LoadingIntro /></div> } 
                        } }}
                        <Topbar/>
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
                            <div class="indexLayout">
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
                                         <ContactRecords profile=profile_clone.clone()   is_ready=is_ready   />
                                     </div>
                                     <div class="about">
                                         <h2>About Me</h2>
                                       <div class="aboutDetail" inner_html={profile.about.clone() }>  </div>
                                     </div>
                                     <div class="skills">
                                         <h2>Skills</h2>
                                         <div>
                                             <SkillRecords
                                                 skills=skills
                                                 is_edit=false
                                             />
                                         </div>
                                     </div>
                                     <div class="skills">
                                     <h2>Languages</h2>
                                     <div>
                                         <LanguageRecords
                                             languages=languages
                                             is_edit=false
                                         />
                                     </div>
                                 </div>
                                 </section>
                                 <HomeTabs 
                                 is_ready=is_ready
                                     experiences={profile.experiences.clone().unwrap_or_default()} 
                                     portfolios={profile.portfolios.clone().unwrap_or_default()}
                                     educations={profile.educations.clone().unwrap_or_default()}
                                 />
                             </div> </div>
                        }   },
                    Some(Err(e)) => view! { 
                        <div class="indexLayout">
                            <div>"Error loading profile: "{e.to_string()}</div>
                        </div> 
                    },
                    None => view! { 
                        <div class="indexLayout">
                            <div>"LoadingIntro..."</div>
                        </div> 
                    }
                }
            }}
        </Suspense>
    }
}
