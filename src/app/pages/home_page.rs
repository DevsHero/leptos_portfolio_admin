use crate::app::{
    components::{ Dialog, HomeContacts, LanguageChips, Loading, SelectTab, SkillChips, Topbar },
    server::api::get_profile,
    utils::calculate_age,
};
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let get_profile_info = Resource::new(
        || (),
        move |_| async move { get_profile().await }
    );
    let (is_ready, set_is_ready) = create_signal(false);

    create_effect(move |_| {
        set_is_ready.set(true);
    });
    view! {
        
        <Suspense fallback=Loading>
            {move || { 
                match get_profile_info.get() {
                    Some(Ok(profile)) => {
                        let (skills, _) = create_signal(profile.skills.clone().unwrap_or_default());
                        let (languages, _) = create_signal(profile.languages.clone().unwrap_or_default());
                        let (birth_date, set_birth_date) = create_signal(String::new());
                        let (open_dialog, set_open_dialog) = create_signal(false);
                    
                        let avatar =  profile.avatar.clone();
                        
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
                                         <HomeContacts  use_pdf=profile.pdf.use_pdf    is_ready=is_ready contacts={profile.contacts.clone().unwrap_or_default()} />
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
                                 />
                             </div>
                        }
                    },
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
