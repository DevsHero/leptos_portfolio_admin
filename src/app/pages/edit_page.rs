use crate::app::components::{ Experience, SkillChips };
use crate::app::models::{ Profile, Skill };
use crate::app::server::api::{ get_profile, update_profile };
use crate::app::utils::format_date_for_input;

use leptos::*;
use web_sys::SubmitEvent;

#[component]
pub fn EditPage() -> impl IntoView {
    let get_profile_info = Resource::new(
        || (),
        |_| async move { get_profile().await }
    );
    view! { <Suspense fallback=move || {
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
            let (experiences, set_experiences) = create_signal(profile.experiences);
            let (skill_name, set_skill_name) = create_signal(String::new());
            let (skill_level, set_skill_level) = create_signal(String::from("basic"));
            let (skills, set_skills) = create_signal(profile.skills.unwrap_or_else(Vec::new));
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
                skills:Some(skills.get()),
                experiences :  experiences.get()
            };
            update_profile_action.dispatch(updated_profile);
        };
        create_effect(move |_| {
                if let Some(Ok(_)) = update_profile_action.value().get() {
                    // Refresh data after successful update
                    get_profile_info.refetch();
                }
            });
            let add_skill = move |_| {
                if !skill_name.get().is_empty() {
                    let new_skill = Skill {
                        name: skill_name.get(),
                        level: skill_level.get(),
                    };
                    set_skills.update(|skills| skills.push(new_skill));
                    set_skill_name.set(String::new()); // Clear input
                }
                set_is_update_skill(true)
            };
            let delete_skill = move |index: usize| {
                set_skills.update(|skills| {
                    skills.remove(index);
                });
            };
            view! {
                <main class="edit-page">
                    <div class="edit-container">
                        <h1>"Edit Profile"</h1>
                        <form on:submit=on_submit class="edit-form">
                            <div class="formGroup">
                            <img src=avatar class="avatar-preview  mx-auto items-center justify-center align-center" alt="Avatar preview" />
                                <label for="avatar">"Avatar URL"</label>
                                <input
                                    type="text"
                                    id="avatar"
                                    prop:value=avatar
                                    on:input=move |ev| {
                                        set_avatar(event_target_value(&ev));
                                    }
                                />
                            </div>
                            <div class="formRow">
                                <div class="formGroup">
                                    <label for="first_name">"First Name"</label>
                                    <input
                                        type="text"
                                        id="first_name"
                                        prop:value=first_name
                                        on:input=move |ev| {
                                            set_first_name(event_target_value(&ev));
                                        }
                                    />
                                </div>
                                <div class="formGroup">
                                    <label for="last_name">"Last Name"</label>
                                    <input
                                        type="text"
                                        id="last_name"
                                        prop:value=last_name
                                        on:input=move |ev| {
                                            set_last_name(event_target_value(&ev));
                                        }
                                    />
                                </div>
                            </div>
                            <div class="formGroup">
                                <label for="nick_name">"Nickname"</label>
                                <input
                                    type="text"
                                    id="nick_name"
                                    prop:value=nick_name
                                    on:input=move |ev| {
                                        set_nick_name(event_target_value(&ev));
                                    }
                                />
                            </div>
                            <div class="formRow">
                                <div class="formGroup">
                                    <label for="gender">"Gender"</label>
                                    <select
                                        id="gender"
                                        prop:value=gender
                                        on:change=move |ev| {
                                            set_gender(event_target_value(&ev));
                                        }
                                    >
                                        <option value="Male">"Male"</option>
                                        <option value="Female">"Female"</option>
                                        <option value="Other">"Other"</option>
                                    </select>
                                </div>
                                <div class="formGroup">
                                    <label for="birth_date">"Birth Date"</label>
                                    <input
                    type="date"
                    id="birth_date"
                        prop:value=birth_date
                        on:input=move |ev| {
                         set_birth_date(event_target_value(&ev));
                                }
                        />
                                </div>
                            </div>
                            <div class="formGroup">
                                <label for="role">"Role"</label>
                                <input
                                    type="text"
                                    id="role"
                                    prop:value=role
                                    on:input=move |ev| {
                                        set_role(event_target_value(&ev));
                                    }
                                />
                            </div>
                            <div class="formGroup">
                                <label for="nationality">"Nationality"</label>
                                <input
                                    type="text"
                                    id="nationality"
                                    prop:value=nationality
                                    on:input=move |ev| {
                                        set_nationality(event_target_value(&ev));
                                    }
                                />
                            </div>
                            <div class="formGroup">
                                <label for="about">"About"</label>
                                <textarea
                                id="about"
                                prop:value=about
                                on:input=move |ev| {
                                    set_about(event_target_value(&ev));
                                }
                            ></textarea>
                            </div>
                        <div class="skills-section">
                            <h2>"Skills"</h2>
                            // Add new skill form
                            <div class="formRow">
                                <div class="formGroup">
                                    <label for="skill_name">"Skill Name"</label>
                                    <input
                                        type="text"
                                        id="skill_name"
                                       prop:value=move || skill_name.get()
                                        on:input=move |ev| {
                                            set_skill_name(event_target_value(&ev));
                                        }
                                    />
                                </div>
                                <div class="formGroup">
                                    <label for="skill_level">"Level"</label>
                                    <select
                                        id="skill_level"
                                        prop:value=skill_level
                                        on:change=move |ev| {
                                            set_skill_level(event_target_value(&ev));
                                        }
                                    >
                                        <option value="Basic">"Basic"</option>
                                        <option value="Middle">"Middle"</option>
                                        <option value="Expert">"Expert"</option>
                                    </select>
                                    <button
                                    type="button"
                                    class="addButton"
                                    on:click=add_skill
                                >
                                    "Add Skill"
                                </button>
                                </div>
                            </div>
                            <SkillChips
                            is_page=false
                            skills=skills
                            on_delete=Some(Callback::new(move |index| delete_skill(index)))
                           use_delete=true
                        />
                        </div>
                        <div class="skills-section">
                        <h2>"Experience"</h2>
                        // Add new skill form
                            <div class="formGroup">
                                <label for="skill_name">"Company Name"</label>
                                <input
                                    type="text"
                                    id="company_name"
                                   prop:value=move || skill_name.get()
                                    on:input=move |ev| {
                                        set_skill_name(event_target_value(&ev));
                                    }
                                />
                            </div>
                        <div class="formGroup">
                                <label for="skill_level">"Company Logo Url"</label>
                                <input
                                type="text"
                                id="company_logo_url"
                               prop:value=move || skill_name.get()
                                on:input=move |ev| {
                                    set_skill_name(event_target_value(&ev));
                                }
                            />
                            </div>
                        <div class="formGroup">
                            <label for="skill_level">"Position"</label>
                            <input
                            type="text"
                            id="position_name"
                           prop:value=move || skill_name.get()
                            on:input=move |ev| {
                                set_skill_name(event_target_value(&ev));
                            }
                        />
                        </div>
                    <div class="formGroup">
                        <label for="skill_level">"Start Date"</label>
                        <input
                        type="text"
                        id="start_date"
                       prop:value=move || skill_name.get()
                        on:input=move |ev| {
                            set_skill_name(event_target_value(&ev));
                        }
                        />
                        </div>
                    <div class="formGroup">
                        <label for="skill_level">"End Date"</label>
                        <input
                        type="text"
                        id="end_date"
                       prop:value=move || skill_name.get()
                        on:input=move |ev| {
                            set_skill_name(event_target_value(&ev));
                        }
                    />
                    </div>
                    <div class="formGroup">
                    <label for="skill_level">"Describe"</label>
                    <textarea
                    id="about"
                    prop:value=about
                    on:input=move |ev| {
                        set_about(event_target_value(&ev));
                    }
                ></textarea>
                </div>
                                <button
                                type="button"
                                class="addButton"
                                on:click=add_skill
                            >
                                "Add Skill"
                            </button>
                            { experiences.get().unwrap().into_iter().enumerate().map(|(index, experience)| {
                                view! {
                              <Experience is_page=true experience=experience index=(index + 1).to_string()/>               
                                }
                            }).collect::<Vec<_>>() }
                           
                    </div>
                        <div class="formButton">
                        <button
                            type="submit"
                            class="save-button"
                            disabled=is_saving
                        >
                            {move || if is_saving.get() { "Updating..." } else { "Update" }}
                        </button>
                        <button
                            type="button"
                            class="cancel-button"
                            disabled=is_saving
                            on:click=reset_form  // Add the reset_form handler here
                        >
                            "Cancel"
                        </button>
                    </div>
                        </form>
                    </div>
                </main>
            }
        }
    }
    </Suspense>
}
}
