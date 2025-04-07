use crate::app::components::edit_tabs::{
    EditContactTab,
    EditEducationTab,
    EditExperienceTab,
    EditLanguageTab,
    EditPortfolioTab,
    EditSkillTab,
};
use crate::app::components::{
    show_error_toast,
    show_success_toast,
    AccessModes,
    CheckBox,
    EditMenu,
    InputField,
    LoadingIntro,
    RenderTab,
    TextEditor,
    Topbar,
};
use crate::app::models::{ Profile, PDF };
use crate::app::server::api::{ get_profile_api, update_profile_api };
use leptos::*;
use web_sys::SubmitEvent;
#[component]
pub fn EditPage() -> impl IntoView {
    let (is_ready, set_is_ready) = create_signal(false);
    let (profile, set_profile) = create_signal(None);
    let (error, set_error) = create_signal(None);
    let (select_tab, set_select_tab) = create_signal(1);
    let get_profile_api_info = Resource::new(
        || (),
        |_| async move { get_profile_api().await }
    );
    let (is_init, set_is_init) = create_signal(false);
    let (is_verify, set_is_verify) = create_signal(false);

    create_effect(move |_| {
        spawn_local(async move {
            match get_profile_api().await {
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
    view! {     
        <head> <script src="/assets/tinymce-integration.js"></script> </head>
        <main class="editPage"  >
        { move || {    
            if !is_ready.get() {
                view! { <div> <LoadingIntro /></div> }
            } else if let Some(error) = error.get() {
                view! { <div>"Error loading profile: " {error}</div> }
            } else if let Some(profile) = profile.get() {              
                {if is_init.get() { 
                //Profile 
                let (first_name, set_first_name) = create_signal(profile.first_name);
                let (last_name, set_last_name) = create_signal(profile.last_name);
                let (about, set_about) = create_signal(profile.about);
                let (nick_name, set_nick_name) = create_signal(profile.nick_name);
                let (gender, set_gender) = create_signal(profile.gender);
                let (role, set_role) = create_signal(profile.role);
                let (birth_date, set_birth_date) = create_signal(profile.birth_date);
                let (nationality, set_nationality) = create_signal(profile.nationality);
                let (avatar, set_avatar) = create_signal(profile.avatar);
                let (address, set_address) = create_signal(profile.address);
                //PDF
                let (use_pdf, set_use_pdf) = create_signal(profile.pdf.use_pdf);
                let (use_generate, set_use_generate) = create_signal(profile.pdf.use_generate);
                let (pdf_link, set_pdf_link) = create_signal(profile.pdf.pdf_link.unwrap_or_default());
                let (use_about_pdf_version, set_use_about_pdf_version) = create_signal(profile.pdf.use_about_pdf_version);
                let (about_pdf_data, set_about_pdf_data) = create_signal(profile.pdf.about_pdf_data.unwrap_or_default());
                let (use_avatar_pdf_version, set_use_avatar_pdf_version) = create_signal(profile.pdf.use_avatar_pdf_version);
                let (avatar_pdf_url, set_avatar_pdf_url) = create_signal(profile.pdf.avatar_pdf_url.unwrap_or_default());
                let (show_contact, set_show_contact) = create_signal(profile.pdf.show_contact);
                let (show_language, set_show_language) = create_signal(profile.pdf.show_language);
                let (show_about, set_show_about) = create_signal(profile.pdf.show_about);
                let (show_education, set_show_education) = create_signal(profile.pdf.show_education);
                let (show_experience, set_show_experience) = create_signal(profile.pdf.show_experience);
                let (show_portfolio, set_show_portfolio) = create_signal(profile.pdf.show_portfolio);
                let (show_skill, set_show_skill) = create_signal(profile.pdf.show_skill);
                let (show_profile, set_show_profile) = create_signal(profile.pdf.show_profile);
                let (show_avatar, set_show_avatar) = create_signal(profile.pdf.show_avatar);         
              
                let (languages, set_languages) = create_signal(profile.languages.unwrap_or_else(Vec::new));         
                let (educations, set_educations) = create_signal(profile.educations.unwrap_or_else(Vec::new));    //Experience 
                let (experiences, set_experiences) = create_signal(profile.experiences.unwrap_or_else(Vec::new));
                let (skills, set_skills) = create_signal(profile.skills.unwrap_or_else(Vec::new));
                let (portfolios, set_portfolios) = create_signal(profile.portfolios.unwrap_or_else(Vec::new));   
                let (contacts, set_contacts) = create_signal(profile.contacts.unwrap_or_else(Vec::new));    

                let (_is_update_skill, set_is_update_skill) = create_signal(false);
                let (_is_update_experience, set_is_update_experience) = create_signal(false);
                let (_is_update_portfolio, set_is_update_portfolio) = create_signal(false);
                let (_is_update_contact, set_is_update_contact) = create_signal(false);
                let (_is_update_education, set_is_update_education) = create_signal(false);
                let (_is_update_language, set_is_update_language) = create_signal(false);
                let (is_saving, set_is_saving) = create_signal(false);
                let (validate_profile, set_validate_profile) = create_signal(false);
                let (validate_pdf, _set_validate_pdf) = create_signal(false);
               
                let update_profile_action = Action::new(move |profile: &Profile| {
                    set_is_saving.set(true);
                    let profile = profile.clone();
                    async move {
                        let result = update_profile_api(
                            profile , 
                            _is_update_skill.get() ,
                            _is_update_portfolio.get(),
                            _is_update_experience.get(),
                            _is_update_language.get(),
                            _is_update_education.get(),
                        _is_update_contact.get()
                         ).await;
              
                        set_is_saving.set(false);
                        set_is_update_skill(false);
                        set_is_update_experience(false);
                        set_is_update_portfolio(false);
                        set_is_update_contact(false);
                        show_success_toast("Add Update Success", "All information has been updated.");
                        result
                    }
                });
                let reset_form = move |_: web_sys::MouseEvent| {  // Add type annotation here
                    get_profile_api_info.refetch();
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
                    show_error_toast("Update Failed", "Profile Missing required fields.");   
                  }else{
                let updated_profile = Profile {
                    id: profile_id.clone(),
                    first_name: first_name.get(),
                    last_name: last_name.get(),
                    about: about.get(),
                    nick_name: nick_name.get(),
                    pdf: PDF {
                        use_pdf: use_pdf.get(),
                        use_generate: use_generate.get(),
                        pdf_link:Some(pdf_link.get()) ,
                        use_about_pdf_version: use_about_pdf_version.get(),
                        use_avatar_pdf_version: use_avatar_pdf_version.get(),
                        about_pdf_data: Some(about_pdf_data.get()),
                        avatar_pdf_url: Some(avatar_pdf_url.get()),
                        show_contact: show_contact.get(),
                        show_language: show_language.get(),
                        show_about: show_about.get(),
                        show_education:show_education.get(),
                        show_experience: show_experience.get(),
                        show_portfolio: show_portfolio.get(),
                        show_skill: show_skill.get(),
                        show_profile: show_profile.get(),
                        show_avatar:show_avatar.get()
                    },
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
                    languages: Some(languages.get()),
                    educations: Some(educations.get()),
                };
                update_profile_action.dispatch(updated_profile);
            }
            };
            create_effect(move |_| {
                    if let Some(Ok(_)) = update_profile_action.value().get() {
                        // Refresh data after successful update
                        get_profile_api_info.refetch();
                    }
                });
                view! {
                  <div> <Topbar/>
                 <EditMenu  select_tab=select_tab set_select_tab=set_select_tab
                 experiences=experiences
                 portfolios=portfolios
                 skills=skills
                 contacts=contacts
                 educations=educations
                 languages=languages
                 />
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
                              <label id="gender">"Gender"</label>
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
                        //tab2             
                      <EditSkillTab 
                      select_tab=select_tab 
                      set_is_update_skill=set_is_update_skill 
                      set_skills=set_skills 
                      skills=skills  />
                            //tab3          
                      <EditExperienceTab 
                      select_tab=select_tab 
                      set_is_update_experience=set_is_update_experience 
                      set_experiences=set_experiences 
                      experiences=experiences  
                         />
                        //tab4 
                         <EditPortfolioTab
                         select_tab=select_tab 
                         set_is_update_portfolio=set_is_update_portfolio 
                         set_portfolios=set_portfolios 
                         portfolios=portfolios  
                         />
                         //tab5 
                         <EditContactTab
                         select_tab=select_tab 
                         set_is_update_contact=set_is_update_contact 
                         set_contacts=set_contacts 
                         contacts=contacts  
                         />
                         //tab6 
                         <EditEducationTab 
                         select_tab=select_tab 
                         set_is_update_education=set_is_update_education 
                         set_educations=set_educations 
                         educations=educations  
                         />
                          //tab7 
                         <EditLanguageTab 
                         select_tab=select_tab 
                         set_is_update_language=set_is_update_language 
                         set_languages=set_languages 
                         languages=languages  
                         />
                  <RenderTab  no=8 active_page=select_tab > 
                  <div class="editContainer ">
                  <h1>"Edit PDF"</h1>
                
                      <div class="formRow">
                      <CheckBox id="use_pdf"  label= "Export CV PDF" set_value=set_use_pdf get_value=use_pdf /> 
                      {move ||  if use_pdf.get() {Some(view! { <CheckBox id="use_generate"  label= "Html Generate (disable = pdf link)" set_value=set_use_generate get_value=use_generate />})} else {None}}
                      </div>
                      {move || if !use_generate.get() && use_pdf.get()
                        {Some(view! { <InputField input_type="text" id="pdf_link" label="Pdf File Link" validation=validate_profile set_value=set_pdf_link  get_value=pdf_link require=true />})}
                        else {None} } 
                        {move || if use_generate.get() && use_pdf.get() {
               Some(view!{        
               <h2 style="text-align:center; font-weight:bold; margin:15px;">  Generate PDF Configuration </h2>
                
                  <div class="formRow">
               <CheckBox id="show_contact"  label= "Show Contact Section" set_value=set_show_contact get_value=show_contact />
               <CheckBox id="show_language"  label= "Show Language Section" set_value=set_show_language get_value=show_language />
               </div>
               <div class="formRow">
               <CheckBox id="show_about"  label= "Show About Section" set_value=set_show_about get_value=show_about />
               <CheckBox id="show_education"  label= "Show Education Section" set_value=set_show_education get_value=show_education />
               </div>
               <div class="formRow">
               <CheckBox id="show_experience"  label= "Show Experience Section" set_value=set_show_experience get_value=show_experience />
               <CheckBox id="show_portfolio"  label= "Show Porfolio Section" set_value=set_show_portfolio get_value=show_portfolio />
               </div>
               <div class="formRow">
               <CheckBox id="show_skill"  label= "Show Skill Section" set_value=set_show_skill get_value=show_skill />
               <CheckBox id="show_profile"  label= "Show Profile Section" set_value=set_show_profile get_value=show_profile />
               </div>
               <CheckBox id="show_avatar"  label= "Show Avatar Section" set_value=set_show_avatar get_value=show_avatar />
               {move || if use_avatar_pdf_version.get()
                {view! { <div> 
                        {move || if !avatar_pdf_url.get().is_empty()  {                  
                         Some( view! {<img src=avatar_pdf_url class="avatar-preview  mx-auto items-center justify-center align-center" alt="PDF Avatar preview" />})}
                        else{None} }
                    <InputField validation=validate_pdf input_type="text" id="avatar" label="Avatar PDF URL" set_value=set_avatar_pdf_url  get_value=avatar_pdf_url require=false />  
                    
                         </div>  } }
                          
                           else {view! {<div> </div>}} } 
               <CheckBox id="use_avatar_pdf_version"  label= "Use Avatar PDF Version" set_value=set_use_avatar_pdf_version get_value=use_avatar_pdf_version />
               {move || if use_about_pdf_version.get()
                {view! { <div> 
                <TextEditor
                label="About Me (PDF Version)"
                id="about_pdf_data"
                validation=validate_pdf
                disabled=false
                require=true
                get_value=about_pdf_data
                set_value=set_about_pdf_data
                />  
                  </div>  } }
                   
                    else {view! {<div> </div>}} } 
                })}else {None} } 
               
                    <CheckBox id="use_about_pdf_version"  label= "Use About PDF Version" set_value=set_use_about_pdf_version get_value=use_about_pdf_version />
                      </div>
                      </RenderTab>

                  {if is_verify.get()  {
                      view! {   <div class="bottomForm">
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
                       } }
                else{
                  view! {
                          <div> </div>
                  } }}
                  </form></div>
                }
                }   else{
                view! {
                <div>     
                <AccessModes set_is_init=set_is_init set_is_verify=set_is_verify />
                 </div>  
            } }}  }else {
                view! { <div>"No profile data available."</div> }
            }}}
        </main>
    }
}
