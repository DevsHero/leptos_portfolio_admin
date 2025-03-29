use crate::app::components::{
    CheckBox,
    EditContacts,
    EditMenu,
    Education,
    Experience,
    IconDropdown,
    InputArrayField,
    InputField,
    LanguageChips,
    Loading,
    Portfolio,
    RenderTab,
    SkillChips,
    TextEditor,
};
use crate::app::models::portfolio::{ Contact, Experience };
use crate::app::models::{ Education, Language, Portfolio, Profile, Skill, PDF };
use crate::app::server::api::{ get_profile, update_portfolio, verify };

use leptos::*;
use leptos_toaster::{ Theme, Toast, ToastId, ToastOptions, ToastVariant, ToasterPosition, Toasts };
use web_sys::SubmitEvent;

#[component]
pub fn EditPage() -> impl IntoView {
    let (is_ready, set_is_ready) = create_signal(false);
    let (profile, set_profile) = create_signal(None);
    let (error, set_error) = create_signal(None);
    let (select_tab, set_select_tab) = create_signal(1);
    let get_profile_info = Resource::new(
        || (),
        |_| async move { get_profile().await }
    );
    let (is_init, set_is_init) = create_signal(false);
    let (is_verify, set_is_verify) = create_signal(false);
    let (use_password, set_use_password) = create_signal(false);
    let (input_password, set_input_password) = create_signal(String::new());
    let (is_incorrect, set_is_incorrect) = create_signal(false);

    let create_toast = move |title: View, detail: View, varaint: ToastVariant| {
        let toast_id = ToastId::new();
        let toast_context = expect_context::<Toasts>();

        toast_context.toast(
            view! {
                <Toast
                    toast_id
                    variant=varaint
                    theme=Theme::Dark
                    invert=false
                    rich_colors=false
                    title=view! { {title} }.into_view()
                    description=Some(view! {  {detail}}.into_view())
                />
            },
            Some(toast_id),
            Some(ToastOptions {
                dismissible: true,
                duration: Some(std::time::Duration::from_secs(4)),
                position: Some(ToasterPosition::BottomLeft),
            })
        );
    };
    let verify_action = Action::new(move |_| {
        async move {
            let result = verify(input_password.get()).await;
            match result {
                Ok(true) => {
                    set_is_incorrect(false);
                    set_is_verify(true);
                    set_is_init(true);
                    create_toast(
                        (
                            {
                                view! { <p class="toastInfo">"Admin Mode" </p> }
                            }
                        ).into_view(),
                        "Welcome Admin user.".into_view(),
                        ToastVariant::Info
                    );
                }
                _ => {
                    create_toast(
                        (
                            {
                                view! { <p class="toastFail">"Failed" </p> }
                            }
                        ).into_view(),
                        "Incorrect Password.".into_view(),
                        ToastVariant::Error
                    );

                    set_is_incorrect(true);
                }
            }
        }
    });
    create_effect(move |_| {
        spawn_local(async move {
            match get_profile().await {
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
                // Loading state
                view! { <div> <Loading /></div> }
            } else if let Some(error) = error.get() {
                // Error state
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
                let (show_contact, set_show_contact) = create_signal(profile.pdf.show_contact);
                let (show_language, set_show_language) = create_signal(profile.pdf.show_language);
                let (show_about, set_show_about) = create_signal(profile.pdf.show_about);
                let (show_education, set_show_education) = create_signal(profile.pdf.show_education);
                let (show_experience, set_show_experience) = create_signal(profile.pdf.show_experience);
                let (show_portfolio, set_show_portfolio) = create_signal(profile.pdf.show_portfolio);
                let (show_skill, set_show_skill) = create_signal(profile.pdf.show_skill);
                let (show_profile, set_show_profile) = create_signal(profile.pdf.show_profile);
                let (show_avatar, set_show_avatar) = create_signal(profile.pdf.show_avatar);

                //Language         
                let (languages, set_languages) = create_signal(profile.languages.unwrap_or_else(Vec::new));
                let (language_name, set_language_name) = create_signal(String::new());
                let (language_level, set_language_level) = create_signal(String::from("Intermediate"));

                //Education         
                let (educations, set_educations) = create_signal(profile.educations.unwrap_or_else(Vec::new));
                let (institute_name, set_institute_name) = create_signal(String::new());
                let (institute_logo_url, set_institute_logo_url) = create_signal(String::new());
                let (graduated_year, set_graduated_year) = create_signal(String::new());
                let (degree, set_degree) = create_signal(String::new());
                let (institute_address, set_institute_address) = create_signal(String::new());
                let (major, set_major) = create_signal(String::new());
                let (gpa, set_gpa) = create_signal(String::new());
                //Experience 
                let (experiences, set_experiences) = create_signal(profile.experiences.unwrap_or_else(Vec::new));
                let (company_name, set_company_name) = create_signal(String::new());
                let (company_address, set_company_address) = create_signal(String::new());
                let (company_url, set_company_url) = create_signal(String::new());
                let (company_logo_url, set_company_logo_url) = create_signal(String::new());
                let (position_name, set_position_name) = create_signal(String::new());
                let (start_date, set_start_date) = create_signal(String::new());
                let (end_date, set_end_date) = create_signal(String::new());
                let (describe, set_describe) = create_signal(String::new());   
                let (use_describe_pdf_version, set_use_describe_pdf_version) = create_signal(bool::from(false));     
                let (describe_pdf_data, set_describe_pdf_data) = create_signal(String::new());   
                //Skill 
                let (skills, set_skills) = create_signal(profile.skills.unwrap_or_else(Vec::new));
                let (skill_name, set_skill_name) = create_signal(String::new());
                let (skill_level, set_skill_level) = create_signal(String::from("Basic"));
                //Portfolio
                let (portfolios, set_portfolios) = create_signal(profile.portfolios.unwrap_or_else(Vec::new));   
                let (portfolio_name, set_portfolio_name) = create_signal(String::new());
                let (portfolio_link, set_portfolio_link) = create_signal(String::new());
                let (is_opensource, set_is_opensource) = create_signal(false);
                let (portfolio_icon_url, set_portfolio_icon_url) = create_signal(String::new());
                let (portfolio_detail, set_portfolio_detail) = create_signal(String::new());
                let (screenshots_url, set_screenshots_url) = create_signal(vec!["".to_string()]);
                let (stacks, set_stacks) = create_signal(vec!["".to_string()]);
                let (use_portfolio_detail_pdf_version, set_use_portfolio_detail_pdf_version) = create_signal(bool::from(false));     
                let (portfolio_detail_pdf_data, set_portfolio_detail_pdf_data) = create_signal(String::new());   
                
                //Contact
                let (contacts, set_contacts) = create_signal(profile.contacts.unwrap_or_else(Vec::new));
                let (contact_value, set_contact_value) = create_signal(String::new());
                let (contact_icon, set_contact_icon) = create_signal(String::new());
                let (contact_title, set_contact_title) = create_signal(String::new());
                let (use_link, set_use_link) = create_signal(false); 

                let (_is_update_skill, set_is_update_skill) = create_signal(false);
                let (_is_update_experience, set_is_update_experience) = create_signal(false);
                let (_is_update_portfolio, set_is_update_portfolio) = create_signal(false);
                let (_is_update_contact, set_is_update_contact) = create_signal(false);
                let (_is_update_education, set_is_update_education) = create_signal(false);
                let (_is_update_language, set_is_update_language) = create_signal(false);
                let (is_saving, set_is_saving) = create_signal(false);

                let (validate_profile, set_validate_profile) = create_signal(false);
                let (validate_pdf, set_validate_pdf) = create_signal(false);
                let (validate_skill, set_validate_skill) = create_signal(false);
                let (validate_language, set_validate_language) = create_signal(false);
                let (validate_experience, set_validate_experience) = create_signal(false);
                let (validate_portfolio, set_validate_portfolio) = create_signal(false);
                let (validate_contact, set_validate_contact) = create_signal(false);
               let (validate_education, set_validate_education) = create_signal(false);
                let update_profile_action = Action::new(move |profile: &Profile| {
                    set_is_saving.set(true);
                    let profile = profile.clone();
                    async move {
                        let result = update_portfolio(
                            profile , 
                            _is_update_skill.get() ,
                            _is_update_portfolio.get(),
                            _is_update_experience.get(),
                            _is_update_language.get(),
                            _is_update_education.get(),
                        _is_update_contact.get()
                         ).await;
                         // reset fields after update
                        set_is_saving.set(false);
                        set_is_update_skill(false);
                        set_is_update_experience(false);
                        set_is_update_portfolio(false);
                        set_is_update_contact(false);
                        create_toast({view! {<p class="toastSuccess">"Update Success" </p>}}.into_view(), "All information has been updated.".into_view(), ToastVariant::Success);
                     
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
                    create_toast( {view! {<p class="toastFail">"Update Failed" </p>}}.into_view() , "Profile Missing required fields.".into_view(), ToastVariant::Error);
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
                        about_pdf_data: Some(about_pdf_data.get()),
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
                    
                    create_toast({view! {<p class="toastInfo">"Add Skill Success" </p>}}.into_view(), "Skill Added.".into_view(), ToastVariant::Success);
                    }
                    else{
                        create_toast( {view! {<p class="toastFail">"Add Skill Failed" </p>}}.into_view() , "Missing required field.".into_view(), ToastVariant::Error);
                    }
                };
                let add_language = move |_| {
                    set_validate_language.update(|v| *v = !*v);
                    let form_valid = !language_name.get().trim().is_empty();
                    if form_valid {
                        let new_language = Language {
                            name: language_name.get(),
                            level: language_level.get(),
                        };
                        set_languages.update(|languages| languages.push(new_language));
                        set_validate_language.set(false);
                        set_language_name.set(String::new());
                        set_language_level.set(String::from("Intermediate"));
                 
                    set_is_update_language(true);
                    
                    create_toast({view! {<p class="toastInfo">"Add Language Success" </p>}}.into_view(), "Language Added.".into_view(), ToastVariant::Success);
                    }
                    else{
                        create_toast( {view! {<p class="toastFail">"Add Language Failed" </p>}}.into_view() , "Missing required field.".into_view(), ToastVariant::Error);
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
                            company_address: company_address.get(),
                            use_describe_pdf_version: use_describe_pdf_version.get(),
                            describe_pdf_data: Some( describe_pdf_data.get())
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
                        set_describe_pdf_data.set(String::new());
                        set_use_describe_pdf_version.set(bool::from(false));
                        set_is_update_experience(true);
                    create_toast({view! {<p class="toastInfo">"Add Experience Success" </p>}}.into_view(), "Experience Added.".into_view(), ToastVariant::Success);
                    }
                    else{
                        create_toast( {view! {<p class="toastFail">"Add Experience Failed" </p>}}.into_view() , "Missing required field.".into_view(), ToastVariant::Error);
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
                            is_opensource: is_opensource.get(),
                            screenshots_url: screenshots_url.get(),
                            stacks: stacks.get(),
                            use_portfolio_detail_pdf_version: use_portfolio_detail_pdf_version.get(),
                            portfolio_detail_pdf_data: Some( portfolio_detail_pdf_data.get())
                        };
                        set_portfolios.update(|portfolio| portfolio.push(new_portfolio));
                        set_validate_portfolio.set(false);
                        set_portfolio_name.set(String::new());
                        set_portfolio_detail.set(String::new());
                        set_portfolio_icon_url.set(String::new());
                        set_portfolio_detail_pdf_data.set(String::new());
                        set_use_portfolio_detail_pdf_version.set(bool::from(false));
                        set_portfolio_link.set(String::new());
                        set_is_opensource.set(false);
                        set_screenshots_url.set(vec!["".to_string()]);
                        set_stacks.set(vec!["".to_string()]);
                  
                    set_is_update_portfolio(true);

                    create_toast({view! {<p class="toastInfo">"Add Portfolio Success" </p>}}.into_view(), "Portfolio Added.".into_view(), ToastVariant::Success);
                    }
                    else{
                        create_toast( {view! {<p class="toastFail">"Add Portfolio Failed" </p>}}.into_view() , "Missing required field.".into_view(), ToastVariant::Error);
                    }
                };
                let add_education = move |_| {
                    set_validate_education.update(|v| *v = !*v);
                    let form_valid = !institute_name.get().trim().is_empty() && 
                                    !institute_address.get().trim().is_empty() && 
                                    !degree.get().trim().is_empty() && 
                                    !major.get().trim().is_empty() && 
                                    !graduated_year.get().trim().is_empty();
                    if form_valid {
                        let new_education = Education { 
                            institute_name: institute_name.get(),
                            institute_logo_url: institute_logo_url.get(),
                            graduated_year: graduated_year.get(),
                            degree: degree.get(),
                            institute_address: institute_address.get(),
                            major: major.get(),
                            gpa: gpa.get()
                        };
                        set_educations.update(|education| education.push(new_education));
                        set_validate_education.set(false);
                        set_institute_name.set(String::new());
                        set_institute_logo_url.set(String::new());
                        set_graduated_year.set(String::new());
                        set_institute_address.set(String::new());
                        set_degree.set(String::new());
                        set_major.set(String::new());
                        set_gpa.set(String::new());
                    set_is_update_education(true);

                    create_toast({view! {<p class="toastInfo">"Add Education Success" </p>}}.into_view(), "Education Added.".into_view(), ToastVariant::Success);
                    }
                    else{
                        create_toast( {view! {<p class="toastFail">"Add Education Failed" </p>}}.into_view() , "Missing required field.".into_view(), ToastVariant::Error);
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
             
                create_toast({view! {<p class="toastInfo">"Add Contact Success" </p>}}.into_view(), "Contact Added.".into_view(), ToastVariant::Success);
                    }
                    else{
                        create_toast( {view! {<p class="toastFail">"Add Contact Failed" </p>}}.into_view() , "Missing required field.".into_view(), ToastVariant::Error);
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
                 let delete_education= move |index: usize| {
                    set_educations.update(|educations| {
                        educations.remove(index);
                    });
                      set_is_update_education(true)
                };
                 let delete_language= move |index: usize| {
                    set_languages.update(|languages| {
                        languages.remove(index);
                    });
                      set_is_update_language(true)
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
                let edit_language = move |index: usize| {
                    let list = languages.get();
                    if let Some(language) = list.iter().enumerate().find(|(i, _)| *i == index) {
                        let language = language.1.clone(); 
                        set_language_name.set(language.name);
                        set_language_level.set(language.level);
                        delete_language(index);        
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
                        set_describe_pdf_data.set(experience.describe_pdf_data.unwrap_or(String::from("")));
                        set_use_describe_pdf_version.set(experience.use_describe_pdf_version);
                        delete_experience(index);        
                    }  
              
                };
                let edit_education = move |index: usize| {
                    let list = educations.get();
                    if let Some(education) = list.iter().enumerate().find(|(i, _)| *i == index) {
                        let education = education.1.clone(); 
                        set_institute_name.set(education.institute_name);
                        set_institute_logo_url.set(education.institute_logo_url);
                        set_graduated_year.set(education.graduated_year);
                        set_degree.set(education.degree);
                        set_institute_address.set(education.institute_address);
                        set_major.set(education.major);
                        set_gpa.set(education.gpa);
                        delete_education(index);        
                    }  
              
                };
                let edit_portfolio = move |index: usize| {
                    let list = portfolios.get();
                    if let Some(portfolio) = list.iter().enumerate().find(|(i, _)| *i == index) {
                        let portfolio = portfolio.1.clone(); 
                        set_portfolio_name.set(portfolio.portfolio_name);
                        set_portfolio_link.set(portfolio.portfolio_link);
                        set_is_opensource.set(portfolio.is_opensource);
                        set_portfolio_detail.set(portfolio.portfolio_detail);
                        set_portfolio_icon_url.set(portfolio.portfolio_icon_url);
                        set_portfolio_detail_pdf_data.set(portfolio.portfolio_detail_pdf_data.unwrap_or(String::from("")));
                        set_use_portfolio_detail_pdf_version.set(portfolio.use_portfolio_detail_pdf_version);
                        set_stacks.set(portfolio.stacks);
                        set_screenshots_url.set(portfolio.screenshots_url);
                        delete_portfolio(index);        
                    }  
                
                };
             
                let edit_contact = move |index: usize| {
                    let list = contacts.get();
                    if let Some(contact) = list.iter().enumerate().find(|(i, _)| *i == index) {
                        let contact = contact.1.clone(); 
                        set_contact_title.set(contact.contact_title.unwrap_or(String::from("")));
                        set_contact_value.set(contact.contact_value);
                        set_contact_icon.set(contact.contact_icon);
                        set_use_link.set(contact.use_link);
                        delete_contact(index);
                       
                    }  
                };
                view! {
                  <div> 
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
                   
                  <RenderTab  no=2 active_page=select_tab>    
                  <Show when=move || select_tab() == 2>
                  <Suspense fallback=move || view! { <p>"Loading..."</p> }> 
              
                  <div class="editContainer">
                  <h1>"Edit Skill"</h1>             
                  <div class="formRow">   
                      <InputField input_type="text" id="skill_name" validation=validate_skill label="Skill Name" set_value=set_skill_name  get_value=skill_name require=true />        
                      <div class="formGroup">
                          <label id="skill_level">"Level"</label>
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
                view!{
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
                  }
                  }else{
                      view!{ <div></div> }
                  }
              }

              <CheckBox id="use_describe_pdf_version"  label= "Use Job Describe PDF version" set_value=set_use_describe_pdf_version  get_value=use_describe_pdf_version />
              { move ||
                if select_tab() == 3  && use_describe_pdf_version.get() {
              view!{
                <div>  <TextEditor
                label="Job Describe (PDF Version)"
                id="describe_pdf_data"
                validation=validate_experience
                disabled=false
                require=true
                get_value=describe_pdf_data
                set_value=set_describe_pdf_data
            />
            </div>
                }
                }else{
                    view!{ <div></div> }
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
                  {move ||view! { <CheckBox id="is_opensource"  label= "Opensource" set_value=set_is_opensource  get_value=is_opensource />}}
                  <InputField input_type="text" id="portfolio_link" label="Project Link Url" set_value=set_portfolio_link  get_value=portfolio_link require=false />
                  <InputField input_type="text" id="portfolio_icon_url" label="Project Icon Url" set_value=set_portfolio_icon_url  get_value=portfolio_icon_url require=false />
                
                  { move ||
                      if select_tab() == 4  {
                    view!{
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
                      }
                      }else{
                          view!{ <div></div> }
                      }
                  }
                     
           
                  <InputArrayField  id="screenshots_url" label="Screenshots url" set_fields=set_screenshots_url  get_values=screenshots_url require=false />
                  <InputArrayField  id="stacks" label="Project Stack" set_fields=set_stacks  get_values=stacks require=false />
                  <CheckBox id="use_portfolio_detail_pdf_version"  label= "Use Portfolio Detail PDF version" set_value=set_use_portfolio_detail_pdf_version get_value=use_portfolio_detail_pdf_version />
                  { move ||
                    if select_tab() == 4  && use_portfolio_detail_pdf_version.get() {
                  view!{
                    <div>  <TextEditor
                    label="Portfolio Detail (PDF Version)"
                    id="portfolio_detail_pdf_data"
                    validation=validate_experience
                    disabled=false
                    require=true
                    get_value=portfolio_detail_pdf_data
                    set_value=set_portfolio_detail_pdf_data
                />
                </div>
                    }
                    }else{
                        view!{ <div></div> }
                    }
                }
                     
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
                      view! {
                          <div>
                          <InputField input_type="text" id="contact_title" label="Contact Title (Show in dialog)" set_value=set_contact_title  get_value=contact_title require=true />
                          </div>
                      }
                  } else {
                      view! { <div></div> }
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
                  <RenderTab  no=6 active_page=select_tab>
                  <Show when=move || select_tab() == 6>
                  <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                  <div class="editContainer">
                  <h1>"Edit Education"</h1>
                  <InputField validation=validate_education input_type="text" id="institute_name" label="Institute Name" set_value=set_institute_name  get_value=institute_name require=true />
                  <InputField input_type="text" id="institute_logo_url" label="Institute Logo Url" set_value=set_institute_logo_url  get_value=institute_logo_url require=false />
                  <InputField validation=validate_education input_type="text" id="institute_address" label="Institute Address" set_value=set_institute_address  get_value=institute_address require=true />
                  <InputField validation=validate_education input_type="text" id="degree" label="Degree" set_value=set_degree  get_value=degree require=true />
                  <InputField validation=validate_education input_type="text" id="major" label="Major" set_value=set_major  get_value=major require=true />
                  <InputField input_type="text" id="gpa" label="GPA" set_value=set_gpa  get_value=gpa require=false />
                  <InputField validation=validate_education input_type="text" id="graduated_year" label="Graduated Year" set_value=set_graduated_year  get_value=graduated_year require=true />
                  <button
                          type="button"
                          class="addButton"
                          on:click=add_education >
                          "Add Education"
                  </button>
                      <Education  
                      educations=educations  
                      on_delete=Callback::new(move |index| delete_education(index))
                      on_edit=Callback::new(move |index| edit_education(index))
                      is_edit=true/ >
              </div>
              </Suspense>
              </Show>
                  </RenderTab>
                  <RenderTab  no=7 active_page=select_tab>    
                  <Show when=move || select_tab() == 7>
                  <Suspense fallback=move || view! { <p>"Loading..."</p> }> 
              
                  <div class="editContainer">
                  <h1>"Edit Language"</h1>             
                  <div class="formRow">   
                      <InputField input_type="text" id="language_name" validation=validate_language label="Language" set_value=set_language_name  get_value=language_name require=true />        
                      <div class="formGroup">
                          <label id="language_level">"Level"</label>
                          <select
                          class="selectDropdown"
                              id="language_level"
                              prop:value=language_level
                              on:change=move |ev| {
                                  set_language_level(event_target_value(&ev));
                              }>
                              <option value="Basic">"Basic"</option>
                              <option value="Intermediate">"Intermediate"</option>
                              <option value="Proficiency">"Proficiency"</option>
                              <option value="Native">"Native"</option>
                          </select>
                          <button
                          type="button"
                              class="addButton"
                          on:click=add_language >
                          "Add Language"
                      </button>
                      </div>
                  </div>
                  <LanguageChips
                  languages=languages
                  on_delete=Callback::new(move |index| delete_language(index))
                  on_edit=Callback::new(move |index| edit_language(index))
                 is_edit=true />
              </div>
              </Suspense>
              </Show>
                  </RenderTab>
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
               <CheckBox id="use_about_pdf_version"  label= "Use About PDF Version" set_value=set_use_about_pdf_version get_value=use_about_pdf_version />
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
                <div class="selectMode" > <b><h1 style="font-size: 1.5rem;">"Edit Page"</h1></b>
            <div style="display: flex; flex-direction: column; margin-top: 15px; gap: 1rem">
             <b style="font-size: 18px;">Select Access Mode</b>
                <button 
                type="button"
                style="width: 20rem; height: 2.5rem; margin-top: 1rem; color:green;   border-width: 1px;  border-color: green;"
                on:click=move |_| {
                    create_toast({view! {<p class="toastInfo">"Viewer Mode" </p>}}.into_view(), "Welcome Viewer user.".into_view(), ToastVariant::Info);
                  
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
                    view! {
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
                } }
             else{
                view! {
 <div></div>
                }} }                       
                </div>
            } }}  }else {
                // Fallback state: No data available
                view! { <div>"No profile data available."</div> }
            }}}
        </main>
    }
}
