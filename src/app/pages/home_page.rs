use crate::app::{
    components::{ Dialog, HomeContacts, Loading, PdfExportButton, SelectTab, SkillChips },
    server::api::{ get_profile, pdf_export },
    utils::calculate_age,
};
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let (is_ready, set_is_ready) = create_signal(false);
    let (profile, set_profile) = create_signal(None);
    let (error, set_error) = create_signal(None);

    create_effect(move |_| {
        spawn_local(async move {
            match get_profile().await {
                Ok(data) => {
                    set_profile.set(Some(data));
                    set_is_ready.set(true);
                }
                Err(e) => {
                    set_error.set(Some(e.to_string()));
                    set_is_ready.set(true);
                }
            }
        });
    });

    let (skills, set_skills) = create_signal(Vec::new());
    let (birth_date, set_birth_date) = create_signal(String::new());
    let (open_dialog, set_open_dialog) = create_signal(false);

    create_effect(move |_| {
        if let Some(profile) = profile.get() {
            set_skills.set(profile.skills.clone().unwrap_or_default());

            let age = calculate_age(&profile.birth_date);
            set_birth_date.set(age.to_string());
        }
    });

    view! {
        {
            move || {
                if !is_ready.get() {
                    // Loading state
                    view! { <div class="homeLoading"> <Loading /></div> }
                } else if let Some(error) = error.get() {
                    // Error state
                    view! { <div>"Error loading profile: " {error}</div> }
                } else if let Some(profile) = profile.get() {
                    // Success state: Render profile data
                    let avatar = profile.avatar.clone();
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
                                    <HomeContacts contacts={profile.contacts.clone().unwrap_or_default()} />
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
                            </section>
                            <SelectTab 
                                experiences={profile.experiences.clone().unwrap_or_default()} 
                                portfolios={profile.portfolios.clone().unwrap_or_default()}
                            />
                        </div>
                    }
                } else {
                    // Fallback state: No data available
                    view! { <div>"No profile data available."</div> }
                }
            }
        }
    }
}
