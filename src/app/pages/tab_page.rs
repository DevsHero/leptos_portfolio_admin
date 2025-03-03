use crate::app::components::{
    Experience,
    InputField,
    RenderTab,
    SkillChips,
    TextAreaField,
    RichTextEditor,
};
use crate::app::models::portfolio::Experience;
use crate::app::models::{ Profile, Skill };
use crate::app::server::api::{ get_profile, update_portfolio };
use crate::app::utils::format_date_for_input;

use leptos::*;
use web_sys::SubmitEvent;

#[component]
pub fn TabPage() -> impl IntoView {
    let (select_tab, set_select_tab) = create_signal(1);
    let get_profile_info = Resource::new(
        || (),
        |_| async move { get_profile().await }
    );
    view! {
        <Suspense fallback=move || {
            view! { <h1>"Fetching Data..."</h1> }
        }>
        {
            move || {
                
                let profile_data = get_profile_info.get().and_then(Result::ok).unwrap_or_default();
                let profile = profile_data.first().cloned().unwrap_or_default();
                let (error, set_error) = create_signal(None::<String>);
                //Profile input
                let (first_name, set_first_name) = create_signal(profile.first_name);
                let (last_name, set_last_name) = create_signal(profile.last_name);
                let (about, set_about) = create_signal(profile.about);
                let (nick_name, set_nick_name) = create_signal(profile.nick_name);
                let (gender, set_gender) = create_signal(profile.gender);
                let (role, set_role) = create_signal(profile.role);
                let (birth_date, set_birth_date) = create_signal(format_date_for_input(&profile.birth_date));
                let (nationality, set_nationality) = create_signal(profile.nationality);
                let (avatar, set_avatar) = create_signal(profile.avatar);
                //Experience input
                let (company_name, set_company_name) = create_signal(String::new());
                let (company_url, set_company_url) = create_signal(String::new());
                let (company_logo_url, set_company_logo_url) = create_signal(String::new());
                let (position_name, set_position_name) = create_signal(String::new());
                let (start_date, set_start_date) = create_signal(String::new());
                let (end_date, set_end_date) = create_signal(String::new());
                let (describe, set_describe) = create_signal(String::new());
                let (experiences, set_experiences) = create_signal(profile.experiences.unwrap_or_else(Vec::new));
                //Skill input
                let (skill_name, set_skill_name) = create_signal(String::new());
                let (skill_level, set_skill_level) = create_signal(String::from("Basic"));
                let (skills, set_skills) = create_signal(profile.skills.unwrap_or_else(Vec::new));
                
                let (is_update_skill, set_is_update_skill) = create_signal(false);
                let (is_update_experience, set_is_update_experience) = create_signal(false);
                let (is_saving, set_is_saving) = create_signal(false);
                    let update_profile_action = Action::new(move |profile: &Profile| {
                    set_is_saving.set(true);
                    let profile = profile.clone();
                    let get_skills = skills.get();
                    async move {
                        let result = update_portfolio(profile , is_update_skill.get() , get_skills ).await;
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
                    experiences :  Some(experiences.get())
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
                        set_skill_name.set(String::new());
                        set_skill_level.set(String::from("Basic"));
                    }
                    set_is_update_skill(true)
                };
                      let add_experience = move |_| {
              
                    if !company_name.get().is_empty() {
                        let new_experience = Experience {
                            company_name: company_name.get(),
                            company_url: company_url.get(),
                            company_logo_url: company_logo_url.get(),
                            position_name: position_name.get(),
                            start_date: start_date.get(),
                            end_date: end_date.get(),
                            describe: describe.get(),
                        };
                        set_experiences.update(|experiences| experiences.push(new_experience));
                        set_company_name.set(String::new()); 
                        set_company_logo_url.set(String::new()); 
                        set_company_url.set(String::new()); 
                        set_position_name.set(String::new()); 
                        set_start_date.set(String::new()); 
                        set_end_date.set(String::new()); 
                        set_describe.set(String::new()); 
                    }
                    set_is_update_experience(true)
                };
                let delete_skill = move |index: usize| {
                    set_skills.update(|skills| {
                        skills.remove(index);
                    });
                };
                let delete_experience = move |index: usize| {
                    set_experiences.update(|experiences| {
                        experiences.remove(index);
                    });
                };
                view! {
                    <main class="tabPage">
                        <div class="tabSectionSelector">
                            <button
                                class=move || {
                                    if select_tab() == 1 { "tabsTitle active" } else { "tabsTitle" }
                                }
                                on:click=move |_| set_select_tab(1)
                            >
                               Profile
                            </button>
                            <button
                                class=move || {
                                    if select_tab() == 2 { "tabsTitle active" } else { "tabsTitle" }
                                }
                                on:click=move |_| set_select_tab(2)
                            >
                                Skill
                            </button>
                            <button
                            class=move || {
                                if select_tab() == 3 { "tabsTitle active" } else { "tabsTitle" }
                            }
                            on:click=move |_| set_select_tab(3)
                        >
                            Experience
                        </button>
                        <button
                        class=move || {
                            if select_tab() == 4 { "tabsTitle active" } else { "tabsTitle" }
                        }
                        on:click=move |_| set_select_tab(4)
                    >
                        Portfolio
                    </button>
                    <button
                    class=move || {
                        if select_tab() == 5 { "tabsTitle active" } else { "tabsTitle" }
                    }
                    on:click=move |_| set_select_tab(5)
                >
                    contact
                </button>
                        </div>
                        <form on:submit=on_submit >
                        <div class="edit-form">
                        <RenderTab is_page=true no=1 active_page=select_tab >
                      
                        <div class="edit-container">
                        <h1>"Edit Profile"</h1>
                        
                             <img src=avatar class="avatar-preview  mx-auto items-center justify-center align-center" alt="Avatar preview" />
                                <InputField  id="avatar" label="Avatar URL" set_field=set_avatar  get_value=avatar require=false />  
                           
                            <div class="formRow">
                                <InputField  id="first_name" label="First Name" set_field=set_first_name  get_value=first_name require=true />
                                <InputField  id="last_name" label="Last Name" set_field=set_last_name  get_value=last_name require=true />
                            </div>

                            <div class="formRow">
                            <InputField  id="nick_name" label="Nick Name" set_field=set_nick_name  get_value=nick_name require=false />
                            <InputField  id="nationality" label="Nationality" set_field=set_nationality  get_value=nationality require=true />
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
                            <InputField  id="role" label="Job Role" set_field=set_role  get_value=role require=true />
                            <TextAreaField  id="about" label="About" set_field=set_about  get_value=about require=true />
                    
                            </div>
                            </RenderTab>
                        <RenderTab is_page=true no=2 active_page=select_tab>
                        
                        <div class="edit-container">
                        <h1>"Edit Skill"</h1>
                        // Add new skill form
                        <div class="formRow">
                        
                            <InputField  id="skill_name" label="Skill Name" set_field=set_skill_name  get_value=skill_name require=true />
                         
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
                        is_page=true
                        skills=skills
                        on_delete=Some(Callback::new(move |index| delete_skill(index)))
                       use_delete=true
                    />
                    </div>
                        </RenderTab>
                        <RenderTab is_page=true no=3 active_page=select_tab>
                        <div class="edit-container">
                        <h1>"Edit Experience"</h1>
                        // Add new skill form
                        <InputField  id="company_name" label="Company Name" set_field=set_company_name  get_value=company_name require=true />
                        <InputField  id="company_logo_url" label="Company Logo Url" set_field=set_company_logo_url  get_value=company_logo_url require=true />
                        <InputField  id="position_name" label="Position Name" set_field=set_position_name  get_value=position_name require=true />
                        <InputField  id="start_date" label="Start Date" set_field=set_start_date  get_value=start_date require=true />
                        <InputField  id="end_date" label="End Date" set_field=set_end_date  get_value=end_date require=true />
                       
                  
                
                    <TextAreaField  id="describe" label="Describe" set_field=set_describe  get_value=describe require=false />
                       
              
                                <button
                                type="button"
                                class="addButton"
                                on:click=add_experience
                            >
                                "Add Experience"
                            </button>
                    
                              <Experience  
                              is_page = true 
                              experiences=experiences
                              on_delete=Some(Callback::new(move |index| delete_experience(index)))
                              use_delete=true
                              />               
                                
                        
                           
                    </div>
                        </RenderTab>
                 
                        <RenderTab is_page=true no=4 active_page=select_tab>
                        <div class="edit-container">
                        <h1>"Edit Portfolio"</h1>
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
                                on:click=add_experience
                            >
                                "Add Experience"
                            </button>
                          
                    </div>
                        </RenderTab>
                        </div>
                        <div class="formButton">
                        <button
                            type="submit"
                            class="updateButton"
                            disabled=is_saving
                        >
                            {move || if is_saving.get() { "Updating..." } else { "Update" }}
                        </button>
                        <button
                            type="button"
                            class="cancelButton"
                            disabled=is_saving
                            on:click=reset_form  // Add the reset_form handler here
                        >
                            "Cancel"
                        </button>
                    </div>
                        </form>
                    </main>
                }
        }}
        </Suspense>
    }
}
