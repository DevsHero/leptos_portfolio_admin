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
                                    <div class="aboutMe">
                                        <span class="avatar">
                                            <a href={profile.avatar.clone()} target="_blank">
                                                <span class="avatar">
                                                    <img alt="avatar" src={profile.avatar.clone()} width="80" height="80" />
                                                </span>
                                            </a>
                                            <div class="details">
                                            <h1>{profile.first_name.clone()}" "{profile.last_name.clone()}" ("{profile.nick_name.clone()}")"</h1>
                                      
                                                <p>{profile.role.clone()}</p>
                                                <div class="info-row">
                                                    <p>Age: {birth_date}</p>
                                                    <p>Gender: {profile.gender.clone()}</p>
                                                </div>
                                                <p>Nationality: {profile.nationality.clone()}</p>
                                            </div>
                                        </span>
                                        <HomeContacts contacts={profile.contacts.clone().unwrap_or_default()} />
                                    </div>
                                    <div class="description">
                                        <h2>About me</h2>
                                        <p>{profile.about.clone()}</p>
                                    </div>
                                    <div class="skills">
                                        <h3>Skills</h3>
                                        <div>
                                            <SkillChips
                                                is_page=false
                                                skills=skills
                                                on_delete=None
                                                use_delete=false
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
