use crate::app::{
    components::{ Dialog, HomeContacts, Loading, SelectTab, SkillChips },
    server::api::get_profile,
    utils::calculate_age,
};
use leptos::{ either::{ Either, EitherOf3 }, prelude::* };

#[component]
pub fn HomePage() -> impl IntoView {
    let get_profile_info = Resource::new(
        || (),
        move |_| async move { get_profile().await }
    );

    (
        view! {
        <Suspense fallback=Loading>
            { move || Suspend::new(async move {
                match get_profile_info.get() {
                    Some(Ok(profile)) =>  EitherOf3::A( {
                        let (skills, _) = signal(profile.skills.clone().unwrap_or_default());
                        let (birth_date, set_birth_date) = signal(String::new());
                        let (open_dialog, set_open_dialog) = signal(false);
                        let avatar =  profile.avatar.clone();                      
                        Effect::new(move |_| {
                        let age = calculate_age(&profile.birth_date);
                            set_birth_date.set(age.to_string());
                        });      
                        view! {
                            <div class="indexLayout">
                           { move || { 
                            if open_dialog.get() { 
                                let clone_avatar =  profile.avatar.clone();
                              Either::Left(  view!  {<div  on:click=move |_| {
                                set_open_dialog.set(!open_dialog.get()); }>
                                <Dialog children_only=true >
                                <img alt="avatar" src={clone_avatar.clone()} />
                            </Dialog>
                                </div>})}
                            else {
                                Either::Right(())
                            } 
                           }  }
                        
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
                    }),
                    Some(Err(e)) =>  EitherOf3::B(view! { 
                        <div class="indexLayout">
                            <div>"Error loading profile: "{e.to_string()}</div>
                        </div> 
                    }),
                    None => EitherOf3::C( view! { 
                        <div class="indexLayout">
                            <div>"Loading..."</div>
                        </div> 
                    })
                }
            })}
        </Suspense>
    }
    ).into_any()
}
