use crate::app::components::{
    CheckBox,
    EditContacts,
    Experience,
    IconDropdown,
    InputArrayField,
    InputField,
    Loading,
    Portfolio,
    RenderTab,
    SkillChips,
    TextEditor,
};
use crate::app::models::portfolio::{ Contact, Experience };
use crate::app::models::{ Profile, Skill, Portfolio };
use crate::app::server::api::{ get_profile, update_portfolio, verify };
use leptos::either::{ Either, EitherOf3 };
use leptos::prelude::*;
use web_sys::SubmitEvent;
use leptoaster::{ expect_toaster, ToastBuilder, ToastLevel, ToastPosition };
#[component]
pub fn EditPage() -> impl IntoView {
    let (select_tab, set_select_tab) = signal(1);
    let get_profile_info = Resource::new(
        || (),
        |_| async move { get_profile().await }
    );
    let (is_init, set_is_init) = signal(false);
    let (is_verify, set_is_verify) = signal(false);
    let (use_password, set_use_password) = signal(false);
    let (input_password, set_input_password) = signal(String::new());
    let (is_incorrect, set_is_incorrect) = signal(false);

    let verify_action = Action::new(move |_| {
        async move {
            let result = verify(input_password.get()).await;
            match result {
                Ok(true) => {
                    set_is_incorrect(false);
                    set_is_verify(true);
                    set_is_init(true);
                    let toaster = expect_toaster();
                    toaster.toast(
                        ToastBuilder::new("Admin mode is Activate")
                            .with_level(ToastLevel::Info)
                            .with_expiry(Some(3_000))
                            .with_position(ToastPosition::TopRight)
                    );
                }
                _ => {
                    let toaster = expect_toaster();
                    toaster.toast(
                        ToastBuilder::new("Incorrect Password.")
                            .with_level(ToastLevel::Error)
                            .with_expiry(Some(3_000))
                            .with_position(ToastPosition::TopRight)
                    );

                    set_is_incorrect(true);
                }
            }
        }
    });
    (
        view! {     
        <script src="/assets/tinymce-integration.js"></script>
        <main class="editPage"  >
     
        <Suspense fallback=Loading>
        { move || Suspend::new(async move {  
         
            match get_profile_info.get() {
                Some(Ok(profile)) => EitherOf3::A({                    
                {if is_init.get() { 
                      //Profile 
                let (first_name, set_first_name) = signal(profile.first_name);
                let (last_name, set_last_name) = signal(profile.last_name);
                let (about, set_about) = signal(profile.about);
                let (nick_name, set_nick_name) = signal(profile.nick_name);
                let (gender, set_gender) = signal(profile.gender);
                let (role, set_role) = signal(profile.role);
                let (birth_date, set_birth_date) = signal(profile.birth_date);
                let (nationality, set_nationality) = signal(profile.nationality);
                let (avatar, set_avatar) = signal(profile.avatar);
                let (address, set_address) = signal(profile.address);
                //Experience 
                    let (experiences, set_experiences) = signal(profile.experiences.unwrap_or_else(Vec::new));
                let (company_name, set_company_name) = signal(String::new());
                let (company_address, set_company_address) = signal(String::new());
                let (company_url, set_company_url) = signal(String::new());
                let (company_logo_url, set_company_logo_url) = signal(String::new());
                let (position_name, set_position_name) = signal(String::new());
                let (start_date, set_start_date) = signal(String::new());
                let (end_date, set_end_date) = signal(String::new());
                let (describe, set_describe) = signal(String::new());      
                //Skill 
                let (skills, set_skills) = signal(profile.skills.unwrap_or_else(Vec::new));
                let (skill_name, set_skill_name) = signal(String::new());
                let (skill_level, set_skill_level) = signal(String::from("Basic"));
                //Portfolio
                let (portfolios, set_portfolios) = signal(profile.portfolios.unwrap_or_else(Vec::new));   
                let (portfolio_name, set_portfolio_name) = signal(String::new());
                let (portfolio_link, set_portfolio_link) = signal(String::new());
                let (is_private, set_is_private) = signal(false);
                let (portfolio_icon_url, set_portfolio_icon_url) = signal(String::new());
                let (portfolio_detail, set_portfolio_detail) = signal(String::new());
                let (screenshots_url, set_screenshots_url) = signal(vec!["".to_string()]);
                let (stacks, set_stacks) = signal(vec!["".to_string()]);
                //Contact
                let (contacts, set_contacts) = signal(profile.contacts.unwrap_or_else(Vec::new));
                let (contact_value, set_contact_value) = signal(String::new());
                let (contact_icon, set_contact_icon) = signal(String::new());
                let (contact_title, set_contact_title) = signal(String::new());
                let (use_link, set_use_link) = signal(false); 

                let (_is_update_skill, set_is_update_skill) = signal(false);
                let (_is_update_experience, set_is_update_experience) = signal(false);
                let (_is_update_portfolio, set_is_update_portfolio) = signal(false);
                let (_is_update_contact, set_is_update_contact) = signal(false);
                let (is_saving, set_is_saving) = signal(false);

                let (validate_profile, set_validate_profile) = signal(false);
                let (validate_skill, set_validate_skill) = signal(false);
                let (validate_experience, set_validate_experience) = signal(false);
                let (validate_portfolio, set_validate_portfolio) = signal(false);
                let (validate_contact, set_validate_contact) = signal(false);
                 
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
                         let toaster = expect_toaster();
                        toaster.toast(
                            ToastBuilder::new("Update Success.")
                                .with_level(ToastLevel::Success) 
                                .with_expiry(Some(3_000))
                                .with_position(ToastPosition::TopRight)
                        ) ;
                        result
                    }
                });
                let reset_form = move |_: web_sys::MouseEvent| {  // Add type annotation here
                    get_profile_info.refetch();
                };
                let profile_id = profile.id.clone();
                let on_submit = move |ev: SubmitEvent| {
                ev.prevent_default();
                set_validate_profile.update(|v| *v = !*v);
                let form_valid = !first_name.get().trim().is_empty() && 
                                 !last_name.get().trim().is_empty()&& 
                                 !about.get().trim().is_empty()&& 
                                 !role.get().trim().is_empty()&& 
                                 !birth_date.get().trim().is_empty()&& 
                                 !nationality.get().trim().is_empty()&& 
                                 !address.get().trim().is_empty();
                if !form_valid {
                     let toaster = expect_toaster();
                    toaster.toast(
                        ToastBuilder::new("Update Failed Profile Missing required fields.")
                            .with_level(ToastLevel::Error) 
                            .with_expiry(Some(3_000))
                            .with_position(ToastPosition::TopRight)
                    ) ;
                }else{
                let updated_profile = Profile {
                    id: profile_id.clone(),
                    first_name: first_name.get(),
                    last_name: last_name.get(),
                    about: about.get(),
                    nick_name: nick_name.get(),
                    gender: gender.get(),
                    role: role.get(),
                    birth_date: birth_date.get(),
                    nationality: nationality.get(),
                    avatar: avatar.get(),
                    address: address.get(),
                    skills:Some(skills.get()),
                    experiences :  Some(experiences.get()),
                    portfolios: Some(portfolios.get()),
                    contacts: Some(contacts.get()),
                };
                update_profile_action.dispatch(updated_profile);
            }
            };
            Effect::new(move |_| {
                    if let Some(Ok(_)) = update_profile_action.value().get() {
                        // Refresh data after successful update
                        get_profile_info.refetch();
                    }
                });
                let add_skill = move |_| {
                    set_validate_skill.update(|v| *v = !*v);
                    let form_valid = !skill_name.get().trim().is_empty();
                    if form_valid {
                        let new_skill = Skill {
                            name: skill_name.get(),
                            level: skill_level.get(),
                        };
                        set_skills.update(|skills| skills.push(new_skill));
                        set_validate_skill.set(false);
                        set_skill_name.set(String::new());
                        set_skill_level.set(String::from("Basic"));
                 
                    set_is_update_skill(true);
                    
                    let toaster = expect_toaster();
                    toaster.toast(
                        ToastBuilder::new("Add Skill Success")
                            .with_level(ToastLevel::Info) 
                            .with_expiry(Some(3_000))
                            .with_position(ToastPosition::TopRight)
                    ) ;
                    }
                    else{
                        let toaster = expect_toaster();
                        toaster.toast(
                            ToastBuilder::new("Add Skill Failed")
                                .with_level(ToastLevel::Error) 
                                .with_expiry(Some(3_000))
                                .with_position(ToastPosition::TopRight)
                        ) ;
                    }
                };
                let add_experience = move |_| {
                    set_validate_experience.update(|v| *v = !*v);
                    let form_valid = !company_name.get().trim().is_empty() && 
                                     !position_name.get().trim().is_empty()&& 
                                     !describe.get().trim().is_empty()&& 
                                     !start_date.get().trim().is_empty()&& 
                                     !end_date.get().trim().is_empty();
                    if form_valid {
                        let new_experience = Experience {
                            company_name: company_name.get(),
                            company_url: company_url.get(),
                            company_logo_url: company_logo_url.get(),
                            position_name: position_name.get(),
                            start_date: start_date.get(),
                            end_date: end_date.get(),
                            describe: describe.get(),
                            company_address: company_address.get()
                        };
                        set_experiences.update(|experiences| experiences.push(new_experience));
                        set_validate_experience.set(false);
                        set_company_name.set(String::new()); 
                        set_company_logo_url.set(String::new()); 
                        set_company_url.set(String::new()); 
                        set_position_name.set(String::new()); 
                        set_start_date.set(String::new()); 
                        set_end_date.set(String::new()); 
                        set_company_address.set(String::new()); 
                        set_describe.set(String::new()); 
                    
                    set_is_update_experience(true);
              
                     let toaster = expect_toaster();
                    toaster.toast(
                        ToastBuilder::new("Add Experience Success")
                            .with_level(ToastLevel::Info) 
                            .with_expiry(Some(3_000))
                            .with_position(ToastPosition::TopRight)
                    ) ;
                    }
                    else{
                        let toaster = expect_toaster();
                        toaster.toast(
                            ToastBuilder::new("Add Experience Failed")
                                .with_level(ToastLevel::Error) 
                                .with_expiry(Some(3_000))
                                .with_position(ToastPosition::TopRight)
                        ) ;
                    }
                };
                let add_portfolio = move |_| {
                    set_validate_portfolio.update(|v| *v = !*v);
                    let form_valid = !portfolio_name.get().trim().is_empty() && 
                                    !portfolio_detail.get().trim().is_empty();
                    if form_valid {
                        let new_portfolio = Portfolio {
                            index: (portfolios.get().len()  +1 )as u8,
                            portfolio_name: portfolio_name.get(),
                            portfolio_detail: portfolio_detail.get(),
                            portfolio_icon_url: portfolio_icon_url.get(),
                            portfolio_link: portfolio_link.get(),
                            is_private: is_private.get(),
                            screenshots_url: screenshots_url.get(),
                            stacks: stacks.get()
                        };
                        set_portfolios.update(|portfolio| portfolio.push(new_portfolio));
                        set_validate_portfolio.set(false);
                        set_portfolio_name.set(String::new());
                        set_portfolio_detail.set(String::new());
                        set_portfolio_icon_url.set(String::new());
                        set_portfolio_link.set(String::new());
                        set_is_private.set(false);
                        set_screenshots_url.set(vec!["".to_string()]);
                        set_stacks.set(vec!["".to_string()]);
                  
                    set_is_update_portfolio(true);
             
                      let toaster = expect_toaster();
                    toaster.toast(
                        ToastBuilder::new("Add Portfolio Success")
                            .with_level(ToastLevel::Info) 
                            .with_expiry(Some(3_000))
                            .with_position(ToastPosition::TopRight)
                    ) ;
                   
                    }
                    else{
                           let toaster = expect_toaster();
                        toaster.toast(
                            ToastBuilder::new("Add Portfolio Success")
                                .with_level(ToastLevel::Error) 
                                .with_expiry(Some(3_000))
                                .with_position(ToastPosition::TopRight)
                        ) ;

                    }
                };

                let add_contact = move |_| {
                    set_validate_contact.update(|v| *v = !*v);
                    let form_valid = !contact_value.get().trim().is_empty() && 
                                     !contact_icon.get().trim().is_empty();
                    if form_valid {
                        let new_contact = Contact {
                            contact_icon: contact_icon.get(),
                            contact_value: contact_value.get(),
                            contact_title: Some(contact_title.get()),
                            use_link: use_link.get()
                        };
                        set_contacts.update(|contact| contact.push(new_contact));
                        set_validate_contact.set(false);
                        set_contact_icon.set(String::new());
                        set_contact_value.set(String::new());
                        set_contact_title.set(String::new());
                        set_use_link.set(false);
                  
                set_is_update_contact(true);
             
               let toaster = expect_toaster();
                toaster.toast(
                    ToastBuilder::new("Add Contact Success")
                        .with_level(ToastLevel::Info) 
                        .with_expiry(Some(3_000))
                        .with_position(ToastPosition::TopRight)
                ) ;
                    }
                    else{
                        let toaster = expect_toaster();
                        toaster.toast(
                            ToastBuilder::new("Add Contact Failed")
                                .with_level(ToastLevel::Error) 
                                .with_expiry(Some(3_000))
                                .with_position(ToastPosition::TopRight)
                        ) ;
                    }
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
                        for i in (index)..( portfolios.len()) {
                            portfolios[i].index = (i+1) as u8;
                        }
                    });

                      set_is_update_portfolio(true)
                };
                 let delete_contact= move |index: usize| {
                    set_contacts.update(|contacts| {
                        contacts.remove(index);
                    });
                      set_is_update_contact(true)
                };
                let edit_skill = move |index: usize| {
                    let list = skills.get();
                    if let Some(skill) = list.iter().enumerate().find(|(i, _)| *i == index) {
                        let skill = skill.1.clone(); 
                        set_skill_name.set(skill.name);
                        set_skill_level.set(skill.level);
                        delete_skill(index);        
                    }  
                };
                let edit_experience = move |index: usize| {
                    let list = experiences.get();
                    if let Some(experience) = list.iter().enumerate().find(|(i, _)| *i == index) {
                        let experience = experience.1.clone(); 
                        set_company_name.set(experience.company_name);
                        set_company_logo_url.set(experience.company_logo_url);
                        set_position_name.set(experience.position_name);
                        set_start_date.set(experience.start_date);
                        set_end_date.set(experience.end_date);
                        set_describe.set(experience.describe);
                        set_company_address.set(experience.company_address);
                        set_company_url.set(experience.company_url);
                        delete_experience(index);        
                    }  
              
                };
                let edit_portfolio = move |index: usize| {
                    let list = portfolios.get();
                    if let Some(portfolio) = list.iter().enumerate().find(|(i, _)| *i == index) {
                        let portfolio = portfolio.1.clone(); 
                        set_portfolio_name.set(portfolio.portfolio_name);
                        set_company_logo_url.set(portfolio.portfolio_link);
                        set_is_private.set(portfolio.is_private);
                        set_portfolio_detail.set(portfolio.portfolio_detail);
                        set_portfolio_icon_url.set(portfolio.portfolio_icon_url);
                        set_stacks.set(portfolio.stacks);
                        set_screenshots_url.set(portfolio.screenshots_url);
                        delete_portfolio(index);        
                    }  
                
                };
             
                let edit_contact = move |index: usize| {
                    let list = contacts.get();
                    if let Some(contact) = list.iter().enumerate().find(|(i, _)| *i == index) {
                        let contact = contact.1.clone(); 
                        set_contact_title.set(contact.contact_title.unwrap());
                        set_contact_value.set(contact.contact_value);
                        set_contact_icon.set(contact.contact_icon);
                        set_use_link.set(contact.use_link);
                        delete_contact(index);
                       
                    }  
                };
                Either::Left(     view! {
                  <div> 
                
                  <div class="tabSectionSelector" >
                      <button
                      type="button"
                          class=move || {
                              if select_tab() == 1 { "tabsTitle active" } else { "tabsTitle" }
                          }
                          on:click=move |_| set_select_tab(1)  >
                         Profile
                      </button>
                      <button
                      type="button"
                          class=move || {
                              if select_tab() == 2 { "tabsTitle active" } else { "tabsTitle" }
                          }
                          on:click=move |_| set_select_tab(2)   >
                          Skill
                      </button>
                      <button
                      type="button"
                       class=move || {
                          if select_tab() == 3 { "tabsTitle active" } else { "tabsTitle" }
                      }
                      on:click=move |_| set_select_tab(3)  >
                      Experience
                  </button>
                  <button
                  type="button"
                  class=move || {
                      if select_tab() == 4 { "tabsTitle active" } else { "tabsTitle" }
                  }
                  on:click=move |_| set_select_tab(4) >
                  Portfolio
              </button>
              <button
              type="button"
              class=move || {
                  if select_tab() == 5 { "tabsTitle active" } else { "tabsTitle" }
              }
              on:click=move |_| set_select_tab(5)  >
              Contact
          </button>
                  </div>
                  <form on:submit=on_submit >
        
                  <RenderTab  no=1 active_page=select_tab > 
                  <div class="editContainer ">
                  <h1>"Edit Profile"</h1>
                       <img src=avatar class="avatar-preview  mx-auto items-center justify-center align-center" alt="Avatar preview" />
                          <InputField input_type="text" id="avatar" label="Avatar URL" set_value=set_avatar  get_value=avatar require=false />  
                     
                      <div class="formRow" >
                          <InputField input_type="text" id="first_name" label="First Name" set_value=set_first_name validation=validate_profile  get_value=first_name require=true />
                          <InputField input_type="text" id="last_name" label="Last Name" set_value=set_last_name validation=validate_profile get_value=last_name require=true />
                      </div>

                      <div class="formRow">
                      <InputField input_type="text" id="nick_name" label="Nick Name" set_value=set_nick_name  get_value=nick_name require=false />
                      <InputField input_type="text" id="nationality" label="Nationality" validation=validate_profile set_value=set_nationality  get_value=nationality require=true />
                      </div>
                    
                      <div class="formRow">
                          <div class="formGroup" >
                              <label for="gender">"Gender"</label>
                              <select
                              class="selectDropdown"
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
             
               
                  <InputField input_type="date" id="birth_date" label="Birth Date" set_value=set_birth_date validation=validate_profile get_value=birth_date require=true />
               
                    
                      </div>
                      <InputField input_type="text" id="role" label="Job Title" set_value=set_role validation=validate_profile get_value=role require=true />
                      <InputField input_type="text" id="address" label="Address" set_value=set_address validation=validate_profile get_value=address require=true />
                      <TextEditor
                      label="About Me"
                  
                      id="about"
                      validation=validate_profile
                      disabled=false
                      require=true
                      get_value=about
                      set_value=set_about
                  />
                      </div>
                      </RenderTab>
                   
                  <RenderTab  no=2 active_page=select_tab>    
                  <Show when=move || select_tab() == 2>
                  <Suspense fallback=move || view! { <p>"Loading..."</p> }> 
              
                  <div class="editContainer">
                  <h1>"Edit Skill"</h1>             
                  <div class="formRow">   
                      <InputField input_type="text" id="skill_name" validation=validate_skill label="Skill Name" set_value=set_skill_name  get_value=skill_name require=true />        
                      <div class="formGroup">
                          <label for="skill_level">"Level"</label>
                          <select
                          class="selectDropdown"
                              id="skill_level"
                              prop:value=skill_level
                              on:change=move |ev| {
                                  set_skill_level(event_target_value(&ev));
                              }>
                              <option value="Basic">"Basic"</option>
                              <option value="Middle">"Middle"</option>
                              <option value="Expert">"Expert"</option>
                          </select>
                          <button
                          type="button"
                              class="addButton"
                          on:click=add_skill >
                          "Add Skill"
                      </button>
                      </div>
                  </div>
                  <SkillChips
                  skills=skills
                  on_delete=Callback::new(move |index| delete_skill(index))
                  on_edit=Callback::new(move |index| edit_skill(index))
                 is_edit=true />
              </div>
              </Suspense>
              </Show>
                  </RenderTab>
                  <RenderTab  no=3 active_page=select_tab>
                  <Show when=move || select_tab() == 3>
                  <Suspense fallback=move || view! { <p>"Loading..."</p> }> 
                  <div class="editContainer">
                  <h1>"Edit Experience"</h1> 
                  <InputField input_type="text" id="company_name" label="Company Name" validation=validate_experience set_value=set_company_name  get_value=company_name require=true />
                  <InputField input_type="text" id="company_logo_url" label="Company Logo Url" set_value=set_company_logo_url  get_value=company_logo_url require=true />
                  <InputField input_type="text" id="position_name" label="Position Name" validation=validate_experience set_value=set_position_name  get_value=position_name require=true />
                  <InputField input_type="text" id="company_url" label="Company Page Url" set_value=set_company_url  get_value=company_url require=false />
                  <InputField input_type="text" id="company_address" label="Company Address" set_value=set_company_address  get_value=company_address require=false />
                    
                  
                  <div class="formRow">
                  <InputField input_type="date" id="start_date" label="Start Date" validation=validate_experience set_value=set_start_date  get_value=start_date require=true />
                  <InputField input_type="date" id="end_date" label="End Date" validation=validate_experience set_value=set_end_date  get_value=end_date require=true /> 
                  </div>
              { move ||
                  if select_tab() == 3  {
                    Either::Left(  view!{
                  <div>  <TextEditor
                  label="Job Describe"
                  id="describe"
                  validation=validate_experience
                  disabled=false
                  require=true
                  get_value=describe
                  set_value=set_describe
              />
              </div>
                  })
                  }else{
                    Either::Right(())
                  }
              }
                   
                          <button
                          type="button"
                          class="addButton"
                          on:click=add_experience  >
                          "Add Experience"
                      </button>
                        <Experience   
                        experiences=experiences
                        on_delete=Callback::new(move |index| delete_experience(index))
                        on_edit=Callback::new(move |index| edit_experience(index))
                        is_edit=true
                            />                      
              </div>
              </Suspense>
              </Show>
                  </RenderTab>
                  <RenderTab  no=4 active_page=select_tab>
                  <Show when=move || select_tab() == 4>
                  <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                  <div class="editContainer">
                  <h1>"Edit Portfolio"</h1>              
                  <InputField input_type="text" id="portfolio_name" label="Project Name" validation=validate_portfolio set_value=set_portfolio_name  get_value=portfolio_name require=true />
                  <InputField input_type="text" id="portfolio_link" label="Project Link Url" set_value=set_portfolio_link  get_value=portfolio_link require=false />
                  <InputField input_type="text" id="portfolio_icon_url" label="Project Icon Url" set_value=set_portfolio_icon_url  get_value=portfolio_icon_url require=false />
                
                  { move ||
                      if select_tab() == 4  {
                        Either::Left(  view!{
                      <div>         
                      <TextEditor
                      label="Project Detail"
                      id="portfolio_detail"
                      validation=validate_portfolio
                      disabled=false
                      require=true
                      get_value=portfolio_detail
                      set_value=set_portfolio_detail
                      />
                  </div>
                      })
                      }else{
                        Either::Right(())
                      }
                  }
                     
           
                  <InputArrayField  id="screenshots_url" label="Screenshots url" set_fields=set_screenshots_url  get_values=screenshots_url require=false />
                  <InputArrayField  id="stacks" label="Project Stack" set_fields=set_stacks  get_values=stacks require=false />
                         <button
                          type="button"
                          class="addButton"
                          on:click=add_portfolio >
                          "Add Portfolio Project"
                      </button>
                    <Portfolio  
                    portfolios=portfolios
                    is_edit=true
                    set_is_update=set_is_update_portfolio
                    set_portfolios=set_portfolios
                    on_delete=Callback::new(move |index| delete_portfolio(index))
                    on_edit=Callback::new(move |index| edit_portfolio(index))
                    />
              </div>
              </Suspense>
              </Show>
                  </RenderTab>
                  <RenderTab  no=5 active_page=select_tab>
                  <Show when=move || select_tab() == 5>
                  <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                  <div class="editContainer">
                  <h1>"Edit Contact"</h1>
                
                  {move ||view! { <CheckBox id="use_link"  label= "Use link (disable dialog)" set_value=set_use_link  get_value=use_link />}}
                  <IconDropdown validation=validate_contact label="Contact Icon"  get_value=contact_icon  set_value=set_contact_icon require=true  / >
                  {move || {if !use_link.get() {
                    Either::Left(     view! {
                          <div>
                          <InputField input_type="text" id="contact_title" label="Contact Title (Show in dialog)" set_value=set_contact_title  get_value=contact_title require=true />
                          </div>
                      } )
                  } else {
                    Either::Right(())
                  }}}
                  <InputField validation=validate_contact input_type="text" id="contact_value" label="Contact Value" set_value=set_contact_value  get_value=contact_value require=true />
                  
                  <button
                          type="button"
                          class="addButton"
                          on:click=add_contact >
                          "Add Contact"
                  </button>
                      <EditContacts  
                      contacts=contacts  
                      on_delete=Callback::new(move |index| delete_contact(index))
                      on_edit=Callback::new(move |index| edit_contact(index))
                      is_edit=true/ >
              </div>
              </Suspense>
              </Show>
                  </RenderTab>
              
                  {if is_verify.get()  {
                    Either::Left(          view! {   <div class="bottomForm">
                  <button
                      type="submit"
                      class="updateButton"
                      disabled=is_saving >
                      {move || if is_saving.get() { "Updating..." } else { "Update" }}
                  </button>
                  <button
                      type="button"
                      class="cancelButton"
                      disabled=is_saving
                      on:click=reset_form  >
                      "Cancel"
                  </button>
              </div>
                       } )}
               else{
                Either::Right(())
                }}
                  </form></div>
                }
           ) }   else{
                Either::Right(      view! {
                <div class="selectMode" > <b><h1 style="font-size: 1.5rem;">"Edit Page"</h1></b>
            <div style="display: flex; flex-direction: column; margin-top: 15px; gap: 1rem">
             <b style="font-size: 18px;">Select Access Mode</b>
                <button 
                type="button"
                style="width: 20rem; height: 2.5rem; margin-top: 1rem; color:green;   border-width: 1px;  border-color: green;"
                on:click=move |_| {
                       let toaster = expect_toaster();
                    toaster.toast(
                        ToastBuilder::new("Viewer Mode is activate")
                            .with_level(ToastLevel::Info) 
                            .with_expiry(Some(3_000))
                            .with_position(ToastPosition::TopRight)
                    );
                    set_is_init(true);     
                }
                >Viewer Mode "(can't update)"</button>
                <button 
                type="button"
                style="width: 20rem; height: 2.5rem;    border-width: 1px;  border-color: blue;"
                on:click=move |_| {
                     set_use_password(true);
                }
                >Admin Mode</button>
                </div>
                {if use_password.get() {
                    Either::Left(      view! {
                        <div style="width: 20rem; margin-top: 30px;">
                        <InputField input_type="password" id="input_password" label="Admin Password" set_value=set_input_password  get_value=input_password require=true />
                     <p style="color:red;">    {move || if is_incorrect.get() { "Incorrect Password" } else { "" }}</p>
                         <div class="formButton">
                        <button
                            type="button"
                            class="updateButton"
                            on:click=  move |_| {
                                verify_action.dispatch(());
                            }>
                            Verify
                        </button>   
                    </div>  
                        </div>         
                } )}
             else{
                Either::Right(())
            } }                       
                </div>
            }) }}  }),
            Some(Err(e)) => EitherOf3::B(view! { 
                <div class="indexLayout">
                    <p>"Error loading profile: "{e.to_string()}</p>
                </div> 
            }  ),
            None => EitherOf3::C(view! { 
                <div class="indexLayout">
                    <p>"Loading..."</p>
                </div> 
            })} })}
        </Suspense>
        </main>
    }
    ).into_any()
}
