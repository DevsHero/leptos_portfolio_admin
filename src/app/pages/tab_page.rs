use crate::app::components::{ Project, RenderTab, SkillChips };
use crate::app::icons;
use crate::app::models::{ Profile, Skill };
use crate::app::server_functions::portfolio::{ get_profile, get_skill, update_profile };
use chrono::NaiveDateTime;
use leptos_meta::*;
use web_sys::SubmitEvent;
fn format_date_for_input(date_str: &str) -> String {
    // Parse the ISO 8601 date string
    if let Ok(datetime) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%SZ") {
        // Format as YYYY-MM-DD
        datetime.format("%Y-%m-%d").to_string()
    } else {
        // Return a default date if parsing fails
        String::from("2000-01-01")
    }
}
#[component]
pub fn TabPage() -> impl IntoView {
    let get_profile_info = Resource::new_blocking(
        || (),
        move |_| async move { get_profile().await }
    );
    let get_skills_info = Resource::new_blocking(
        || (),
        move |_| async move { get_skill().await }
    );
    view! {
        <Suspense fallback=move || {
            view! { <h1>"Fetching Data..."</h1> }
        }>
        {
            move || {
                let profile_data = get_profile_info.get().and_then(Result::ok).unwrap_or_default();
                let profile = profile_data.first().cloned().unwrap_or_default();
                // Create signals for each field
                let (first_name, set_first_name) = create_signal(profile.first_name);
                let (last_name, set_last_name) = create_signal(profile.last_name);
                let (about, set_about) = create_signal(profile.about);
                let (nick_name, set_nick_name) = create_signal(profile.nick_name);
                let (gender, set_gender) = create_signal(profile.gender);
                let (role, set_role) = create_signal(profile.role);
                let (birth_date, set_birth_date) = create_signal(format_date_for_input(&profile.birth_date));
                let (nationality, set_nationality) = create_signal(profile.nationality);
                let (avatar, set_avatar) = create_signal(profile.avatar);
                let (skill_name, set_skill_name) = create_signal(String::new());
                let (skill_level, set_skill_level) = create_signal(String::from("basic"));
                let (skills, set_skills) = create_signal(Vec::new());
                let (is_update_skill, set_is_update_skill) = create_signal(false);
                let (is_saving, set_is_saving) = create_signal(false);
                    let update_profile_action = Action::new(move |profile: &Profile| {
                    set_is_saving.set(true);
                    let profile = profile.clone();
                    let get_skills = skills.get();
                    async move {
                        let result = update_profile(profile , is_update_skill.get() , get_skills ).await;
                        set_is_saving.set(false);
                        set_is_update_skill(false);
                        result
                    }
                });
                let reset_form = move |_: web_sys::MouseEvent| {  // Add type annotation here
                    get_profile_info.refetch();
                };
             let profile_id = profile.id.clone();
             let on_submit = move |ev: SubmitEvent| {
                ev.prevent_default();
                let updated_profile = Profile {
                    id: profile_id.clone(), // Use the cloned id
                    first_name: first_name.get(),
                    last_name: last_name.get(),
                    about: about.get(),
                    nick_name: nick_name.get(),
                    gender: gender.get(),
                    role: role.get(),
                    birth_date: birth_date.get(),
                    nationality: nationality.get(),
                    avatar: avatar.get(),
                };
                update_profile_action.dispatch(updated_profile);
            };
            create_effect(move |_| {
                    if let Some(Ok(skill_data)) = get_skills_info.get() {
                        set_skills.set(skill_data);
                    }
                    if let Some(Ok(_)) = update_profile_action.value().get() {
                        // Refresh data after successful update
                        get_profile_info.refetch();
                    }
                });
                // let add_skill = move |_| {
                //     if !skill_name.get().is_empty() {
                //         let new_skill = Skill {
                //             name: skill_name.get(),
                //             level: skill_level.get(),
                //         };
                //         set_skills.update(|skills| skills.push(new_skill));
                //         set_skill_name.set(String::new()); // Clear input
                //     }
                //     set_is_update_skill(true)
                // };
                let delete_skill = move |index: usize| {
                    set_skills.update(|skills| {
                        skills.remove(index);
                    });
                };
                let (select_tab, set_tab) = create_signal(1);
                view! {
                    <section class="projectSection">
                        <div class="projectSectionSelector">
                            <button
                                class=move || {
                                    if select_tab() == 1 { "projectsTitle active" } else { "projectsTitle" }
                                }
                                on:click=move |_| set_tab(1)
                            >
                                My projects
                            </button>
                            <button
                                class=move || {
                                    if select_tab() == 2 { "projectsTitle active" } else { "projectsTitle" }
                                }
                                on:click=move |_| set_tab(2)
                            >
                                Contributions
                            </button>
                        </div>
                        <RenderTab no=1 active_page=select_tab>
                            <p>"Nothing to see here yet."</p>
                        </RenderTab>
                        <RenderTab no=2 active_page=select_tab>
                            <Project
                                author=Some("RustLangES")
                                name=".NET - Rust example"
                                description="Example of interoperability between dotnet and Rust"
                                url="https://github.com/gxskpo/dotnet-rust-example/"
                                icon_url="https://avatars.githubusercontent.com/u/74681819?"
                            >
                                <icons::NET />
                                <icons::CSharp />
                                <icons::Rust />
                            </Project>
                            <Project
                                author=Some("RustLangES")
                                name="Resume"
                                description="RustLangES web redesign, built with Astro and tailwind"
                                url="https://github.com/RustLangES/resume"
                                icon_url="https://avatars.githubusercontent.com/u/74681819?"
                            >
                                <icons::Astro />
                            </Project>
                        </RenderTab>
                    </section>
                            }
        }}
        </Suspense>
    }
}
