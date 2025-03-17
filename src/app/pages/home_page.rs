use crate::app::components::SkillChips;
use crate::app::{
    components::{ SelectTab, ThemeButton, HomeContacts, Loading },
    server::api::get_profile,
    utils::calculate_age,
};
use leptos::*;
use leptos_icons::Icon;
use icondata as i;

#[component]
pub fn HomePage() -> impl IntoView {
    let get_profile_info = Resource::new(
        || (),
        move |_| async move { get_profile().await }
    );

    view! {
        <Suspense fallback=Loading>
            {move || {
                // Directly match on the result rather than using .map()
                match get_profile_info.get() {
                    Some(Ok(profile)) => {
                        let (skills, _) = create_signal(profile.skills.clone().unwrap_or_default());
                        let (birth_date, set_birth_date) = create_signal(String::new());
                        
                        create_effect(move |_| {
                            let age = calculate_age(&profile.birth_date);
                            set_birth_date.set(age.to_string());
                        });
                        
                        view! {
                            <main class="indexLayout">
                                <section class="topbar">
                                    <div class="pill">
                                        <button 
                                            disabled
                                            class="topbarButton active">
                                            <Icon icon={i::AiHomeOutlined} />
                                        </button>
                                        <a
                                            href="/edit"
                                            target="_self"
                                            aria-label="Source code"
                                            class="topbarButton"
                                        >
                                            <Icon icon={i::OcGearSm} />
                                        </a>
                                        <ThemeButton />
                                    </div>
                                </section>
                                <section class="info">
                                    <div class="profile">
                                        <span class="avatar">
                                            <a href={profile.avatar.clone()} target="_blank">
                                           
                                                <span class="avatar">
                                                    <img alt="avatar" src={profile.avatar.clone()}   />
                                               
                                                </span>
                                                <p  style="text-align: center; font-weight: bold">  {profile.nick_name.clone()}</p>
                                            </a>
                                            <div class="details">
                                            <h1>{profile.first_name.clone()}" "{profile.last_name.clone()}</h1>
                                      
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
                                                is_page=false
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
                            </main>
                        }
                    },
                    Some(Err(e)) => view! { 
                        <main class="indexLayout">
                            <div>"Error loading profile: "{e.to_string()}</div>
                        </main> 
                    },
                    None => view! { 
                        <main class="indexLayout">
                            <div>"Loading..."</div>
                        </main> 
                    }
                }
            }}
        </Suspense>
    }
}
