use crate::app::components::SkillChips;
use crate::app::{
    components::{ SelectTab, ThemeButton, HomeContacts },
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
    <Suspense fallback = move || {
        view! { <h1 style="margin: 1rem;">"Fetching Data..."</h1> }
    }>
    {
        move || {
            let profile_data = get_profile_info.get().and_then(Result::ok).unwrap_or_default();
            let profile = profile_data.first().cloned().unwrap_or_default();
            let (skills, _set_skills) = create_signal(profile.skills.unwrap_or_else(Vec::new).clone());
            let (birth_date, set_birth_date) = create_signal(String::new());
            Effect::new_isomorphic(move |_| {
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
                                    <a href={profile.avatar.clone()}  target="_blank" >
                                    <span class="avatar">
                                        <img alt="avatar" src={profile.avatar} width="80" height="80" />
                                        </span>
                                        </a>
                                        <div class="details">
                                        <h1>{profile.first_name}" "{profile.last_name}" ("{profile.nick_name}")"</h1>
                                            <p>{profile.role}</p>
                                            <div class="info-row">    <p>"Age: "{birth_date}</p>     <p>"Gender: "{profile.gender}</p> </div>
                                            <p>"Nationality: "{profile.nationality}</p>
                                        </div>
                                    </span>
                                    <HomeContacts contacts=profile.contacts.unwrap_or_else(Vec::new) />
                                </div>
                                <div class="description">
                                    <h2>About me</h2>
                                    <p>{profile.about}</p>
                                </div>
                                <div class="skills">
                                    <h3>Skills</h3>
                                    <div >
                                   
                                <SkillChips
                                is_page=false
                                skills=skills
                                on_delete=None
                                use_delete=false
                                />
                                    </div>
                                </div>
                            </section>
                            <SelectTab experiences=profile.experiences.unwrap() portfolios=profile.portfolios.unwrap()/>
                        </main>
                    }
                }
            }
            </Suspense>
        }
}
