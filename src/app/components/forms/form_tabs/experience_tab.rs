use crate::app::components::forms::{ CheckBox, InputField, TextEditor };
use crate::app::components::records::ExperienceRecords;
use crate::app::components::utils::{ show_error_toast, show_success_toast };
use crate::app::components::layouts::TabRender;
use crate::app::models::Experience;
use leptos::*;

#[component]
pub fn EditExperienceTab(
    experiences: ReadSignal<Vec<Experience>>,
    set_experiences: WriteSignal<Vec<Experience>>,
    set_is_update_experience: WriteSignal<bool>,
    select_tab: ReadSignal<i32>
) -> impl IntoView {
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
    let (validate_experience, set_validate_experience) = create_signal(false);

    let add_experience = move |_| {
        set_validate_experience.update(|v| {
            *v = !*v;
        });
        let form_valid =
            !company_name.get().trim().is_empty() &&
            !position_name.get().trim().is_empty() &&
            !describe.get().trim().is_empty() &&
            !start_date.get().trim().is_empty() &&
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
                describe_pdf_data: Some(describe_pdf_data.get()),
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
            show_success_toast("Add Experience Success", "Experience Added.");
        } else {
            show_error_toast("Add Experience Failed", "Missing required field.");
        }
    };
    let delete_experience = move |index: usize| {
        set_experiences.update(|experiences| {
            experiences.remove(index);
        });
        set_is_update_experience(true)
    };

    let edit_experience = move |index: usize| {
        let list = experiences.get();
        if
            let Some(experience) = list
                .iter()
                .enumerate()
                .find(|(i, _)| *i == index)
        {
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
    view! {
        <TabRender  no=3 active_page=select_tab>
                  <Show when=move || select_tab() == 3>
                  <Suspense fallback=move || view! { <p>"LoadingIntro..."</p> }> 
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
                        <ExperienceRecords
                        experiences=experiences
                        on_delete=Callback::new(move |index| delete_experience(index))
                        on_edit=Callback::new(move |index| edit_experience(index))
                        is_edit=true
                            />                      
                    </div>
                    </Suspense>
                    </Show>
                  </TabRender>
    }
}
