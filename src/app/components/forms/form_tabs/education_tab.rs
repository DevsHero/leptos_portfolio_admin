use crate::app::components::forms::InputField;
use crate::app::components::layouts::TabRender;
use crate::app::components::records::EducationRecords;

use crate::app::components::utils::{ show_error_toast, show_success_toast };
use crate::app::models::Education;
use leptos::*;

#[component]
pub fn EditEducationTab(
    educations: ReadSignal<Vec<Education>>,
    set_educations: WriteSignal<Vec<Education>>,
    set_is_update_education: WriteSignal<bool>,
    select_tab: ReadSignal<i32>
) -> impl IntoView {
    let (institute_name, set_institute_name) = create_signal(String::new());
    let (institute_logo_url, set_institute_logo_url) = create_signal(String::new());
    let (graduated_year, set_graduated_year) = create_signal(String::new());
    let (degree, set_degree) = create_signal(String::new());
    let (institute_address, set_institute_address) = create_signal(String::new());
    let (major, set_major) = create_signal(String::new());
    let (gpa, set_gpa) = create_signal(String::new());
    let (validate_education, set_validate_education) = create_signal(false);
    let add_education = move |_| {
        set_validate_education.update(|v| {
            *v = !*v;
        });
        let form_valid =
            !institute_name.get().trim().is_empty() &&
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
                gpa: gpa.get(),
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
            show_success_toast("Add Education Success", "Education Added.");
        } else {
            show_error_toast("Add Education Failed", "Missing required field.");
        }
    };

    let delete_education = move |index: usize| {
        set_educations.update(|educations| {
            educations.remove(index);
        });
        set_is_update_education(true)
    };
    let edit_education = move |index: usize| {
        let list = educations.get();
        if
            let Some(education) = list
                .iter()
                .enumerate()
                .find(|(i, _)| *i == index)
        {
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
    view! {
        <TabRender  no=6 active_page=select_tab>
        <Show when=move || select_tab() == 6>
        <Suspense fallback=move || view! { <p>"LoadingIntro..."</p> }>
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
            <EducationRecords
            educations=educations  
            on_delete=Callback::new(move |index| delete_education(index))
            on_edit=Callback::new(move |index| edit_education(index))
            is_edit=true/ >
    </div>
    </Suspense>
    </Show>
        </TabRender>
    }
}
