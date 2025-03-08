use crate::app::components::{
    CheckBox,
    EditContacts,
    Experience,
    IconDropdown,
    InputArrayField,
    InputField,
    Portfolio,
    RenderTab,
    SkillChips,
    TextAreaField,
    ThemeButton,
};
use crate::app::models::portfolio::{ Contact, Experience };
use crate::app::models::{ Profile, Skill, Portfolio };
use crate::app::server::api::{ get_profile, update_portfolio, verify };
use crate::app::utils::format_date_for_input;
use leptos_icons::Icon;
use icondata as i;
use leptos::*;
use web_sys::SubmitEvent;

#[component]
pub fn EditPage() -> impl IntoView {
    let (select_tab, set_select_tab) = create_signal(1);
    let get_profile_info = Resource::new(
        || (),
        |_| async move { get_profile().await }
    );

    let (is_init, set_is_init) = create_signal(bool::from(false));
    let (is_verify, set_is_verify) = create_signal(bool::from(false));
    let (use_password, set_use_password) = create_signal(bool::from(false));
    let (input_password, set_input_password) = create_signal(String::new());
    let (is_incorrect, set_is_incorrect) = create_signal(bool::from(false));
    view! {
        <Suspense fallback=move || {
            view! { <h1>"Fetching Data..."</h1> }
        }>
        {
            move || {      
                       
                let profile_data = get_profile_info.get().and_then(Result::ok).unwrap_or_default();
                let profile = profile_data.first().cloned().unwrap_or_default();
      
                //Profile 
                let (first_name, set_first_name) = create_signal(profile.first_name);
                let (last_name, set_last_name) = create_signal(profile.last_name);
                let (about, set_about) = create_signal(profile.about);
                let (nick_name, set_nick_name) = create_signal(profile.nick_name);
                let (gender, set_gender) = create_signal(profile.gender);
                let (role, set_role) = create_signal(profile.role);
                let (birth_date, set_birth_date) = create_signal(format_date_for_input(&profile.birth_date));
                let (nationality, set_nationality) = create_signal(profile.nationality);
                let (avatar, set_avatar) = create_signal(profile.avatar);

                //Experience 
                let (experiences, set_experiences) = create_signal(profile.experiences.unwrap_or_else(Vec::new));
                let (company_name, set_company_name) = create_signal(String::new());
                let (company_url, set_company_url) = create_signal(String::new());
                let (company_logo_url, set_company_logo_url) = create_signal(String::new());
                let (position_name, set_position_name) = create_signal(String::new());
                let (start_date, set_start_date) = create_signal(String::new());
                let (end_date, set_end_date) = create_signal(String::new());
                let (describe, set_describe) = create_signal(String::new());
               
                //Skill 
                let (skills, set_skills) = create_signal(profile.skills.unwrap_or_else(Vec::new));
                let (skill_name, set_skill_name) = create_signal(String::new());
                let (skill_level, set_skill_level) = create_signal(String::from("Basic"));
          
                //Portfolio
                let (portfolios, set_portfolios) = create_signal(profile.portfolios.unwrap_or_else(Vec::new));   
                let (portfolio_name, set_portfolio_name) = create_signal(String::new());
                let (portfolio_link, set_portfolio_link) = create_signal(String::new());
                let (is_private, set_is_private) = create_signal(bool::from(false));
                let (portfolio_icon_url, set_portfolio_icon_url) = create_signal(String::new());
                let (portfolio_detail, set_portfolio_detail) = create_signal(String::new());
                let (screenshots_url, set_screenshots_url) = create_signal(vec!["".to_string()]);
                let (stacks, set_stacks) = create_signal(vec!["".to_string()]);
    
                //Contact
                let (contacts, set_contacts) = create_signal(profile.contacts.unwrap_or_else(Vec::new));
                let (contact_value, set_contact_value) = create_signal(String::new());
                let (contact_icon, set_contact_icon) = create_signal(String::new());
                let (is_href, set_is_href) = create_signal(bool::from(false)); 
                let (_is_update_skill, set_is_update_skill) = create_signal(false);
                    let (_is_update_experience, set_is_update_experience) = create_signal(false);
                let (_is_update_portfolio, set_is_update_portfolio) = create_signal(false);
                let (_is_update_contact, set_is_update_contact) = create_signal(false);
            
                let (is_saving, set_is_saving) = create_signal(false);
                
                let verify_action = Action::new(move |_| {
                    async move { 
                        let result = verify(input_password.get()).await;
                        match result {
                            Ok(true) => {
                                set_is_incorrect(false);
                                set_is_verify(true);
                                set_is_init(true);
                            },
                            _ => {
                                set_is_incorrect(true);
                            },
                        }
                    }
                });
                
                let update_profile_action = Action::new(move |profile: &Profile| {
                    set_is_saving.set(true);
                    let profile = profile.clone();
                    async move {
                        let result = update_portfolio(
                            profile , 
                            _is_update_skill.get() ,
                            _is_update_portfolio.get(),
                            _is_update_experience.get(),
                        _is_update_contact.get()
                         ).await;
                         // reset fields after update
                        set_is_saving.set(false);
                        set_is_update_skill(false);
                        set_is_update_experience(false);
                        set_is_update_portfolio(false);
                        set_is_update_contact(false);
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
                    experiences :  Some(experiences.get()),
                    portfolios: Some(portfolios.get()),
                    contacts: Some(contacts.get()),
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
                let add_portfolio = move |_| {
                    if !portfolio_name.get().is_empty() {
                        let new_portfolio = Portfolio {
                            portfolio_name: portfolio_name.get(),
                            portfolio_detail: portfolio_detail.get(),
                            portfolio_icon_url: portfolio_icon_url.get(),
                            portfolio_link: portfolio_link.get(),
                            is_private: is_private.get(),
                            screenshots_url: screenshots_url.get(),
                            stacks: stacks.get()
                        };
                        set_portfolios.update(|portfolio| portfolio.push(new_portfolio));
                        set_portfolio_name.set(String::new());
                        set_portfolio_detail.set(String::new());
                        set_portfolio_icon_url.set(String::new());
                        set_portfolio_link.set(String::new());
                        set_is_private.set(bool::from(false));
                        set_screenshots_url.set(vec!["".to_string()]);
                        set_stacks.set(vec!["".to_string()]);
                    }
                    set_is_update_portfolio(true)
                };

                let add_contact = move |_| {
                    if !contact_value.get().is_empty() {
                        let new_contact = Contact {
                            contact_icon: contact_icon.get(),
                            contact_value: contact_value.get(),
                            is_href: is_href.get()
                        };
                        set_contacts.update(|contact| contact.push(new_contact));
                        set_contact_icon.set(String::new());
                        set_contact_value.set(String::new());
                        set_is_href.set(bool::from(false));
                    }
                set_is_update_contact(true)
                };
                let delete_skill = move |index: usize| {
                    set_skills.update(|skills| {
                        skills.remove(index);
                    });
                      set_is_update_skill(true)
                };
                let delete_experience = move |index: usize| {
                    set_experiences.update(|experiences| {
                        experiences.remove(index);
                    });
                      set_is_update_experience(true)
                };
                 let delete_portfolio= move |index: usize| {
                    set_portfolios.update(|portfolios| {
                        portfolios.remove(index);
                    });
                      set_is_update_portfolio(true)
                };
                 let delete_contact= move |index: usize| {
                    set_contacts.update(|contacts| {
                        contacts.remove(index);
                    });
                      set_is_update_contact(true)
                };
                {if is_init.get() { 
                view! {
                    <main class="tabPage">
                    <section class="topbar">
                                <div class="pill">
                                <a
                                href="/"
                                target="_self"
                                aria-label="Source code"
                                  class="topbarButton"
                            
                            >
                                    <Icon icon={i::AiHomeOutlined} />
                                    </a>
                                    <button
                                        class="topbarButton active"
                                    disabled
                                    >
                                    <Icon icon={i::OcGearSm} />
                                    </button>
                                    <ThemeButton />
                                </div>
                            </section>
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
                    Contact
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
                                <div class="formGroup" >
                                    <label for="gender">"Gender"</label>
                                    <select
                                    style="height:3.2rem;"
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
                        <InputField  id="portfolio_name" label="Project Name" set_field=set_portfolio_name  get_value=portfolio_name require=true />
                        <InputField  id="portfolio_detail" label="Project Detail" set_field=set_portfolio_detail  get_value=portfolio_detail require=true />
                        <InputField  id="portfolio_link" label="Project Link Url" set_field=set_portfolio_link  get_value=portfolio_link require=false />
                        <InputField  id="portfolio_icon_url" label="Project Icon Url" set_field=set_portfolio_icon_url  get_value=portfolio_icon_url require=false />
                        <InputArrayField  id="screenshots_url" label="Screenshots url" set_fields=set_screenshots_url  get_values=screenshots_url require=false />
                        <InputArrayField  id="stacks" label="Project Stack" set_fields=set_stacks  get_values=stacks require=false />
                               <button
                                type="button"
                                class="addButton"
                                on:click=add_portfolio
                            >
                                "Add Portfolio Project"
                            </button>
                          <Portfolio  
                          portfolios=portfolios
                          use_delete=true
                          on_delete=Some(Callback::new(move |index| delete_portfolio(index)))
                          />
                    </div>
                        </RenderTab>
                        <RenderTab is_page=true no=5 active_page=select_tab>
                        <div class="edit-container">
                        <h1>"Edit Contact"</h1>
                      
                        <CheckBox id="is_href" label= "Use Href Link" set_field=set_is_href  get_value=is_href />
                        <IconDropdown label="Contact Icon"   set_field=set_contact_icon/ >
                        <InputField  id="contact_value" label="Contact Value" set_field=set_contact_value  get_value=contact_value require=true />
                        <button
                                type="button"
                                class="addButton"
                                on:click=add_contact
                            >
                                "Add Contact"
                        </button>
                            <EditContacts  
                            contacts=contacts  
                            on_delete=Some(Callback::new(move |index| delete_contact(index)))
                            use_delete=true
                             / >
                    </div>
                        </RenderTab>
                        </div>
                        {if is_verify.get()  {
                            view! {   <div class="formButton">
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
                             } }
                     else{
                        view! {
                                <div> </div>
                        }
                     }
                    }
                        </form>
                    </main>
                }
        }   else{
            view! {
                <main  > <b><h1 style="font-size: 30px;">"Edit Page"</h1></b>
            <div style="display: flex; flex-direction: column; margin-top: 50px;">
             <b style="font-size: 18px;">Select Access Mode</b>
                <button style="margin-top: 30px; color:green;"
                on:click=move |_| {
                    set_is_init(true);
                   
                }
                
                
                ><b>Viewer Mode "(can't update)"</b></button>
                <button style="margin-top: 30px; color:blue;"
                on:click=move |_| {
                    set_use_password(true);
                }
                ><b>Admin Mode "(can update)"</b></button>
                </div>
                {if use_password.get() {
                    view! {
                        <div style="margin-top: 30px;">
                        <InputField  id="input_password" label="Admin Password" set_field=set_input_password  get_value=input_password require=true />
                     <p style="color:red;">    {move || if is_incorrect.get() { "Incorrect Password" } else { "" }}</p>
                         <div class="formButton">
                        <button
                            type="button"
                            class="updateButton"
                            on:click=  move |_| {
                                verify_action.dispatch(());
                            } 
                        >
                            {move || if is_saving.get() { "Verifying..." } else { "Verify" }}
                        </button>
                      
                    </div>
                        
                        </div>
                }
                }
             else{
                view! {
 <div></div>
                }
             }
            }
                       
                </main>
            }

        }}}
 
    }
        </Suspense>
    }
}
