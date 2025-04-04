use leptos::*;
use crate::app::models::portfolio::Education;
use leptos_icons::Icon;
use icondata as i;
#[component]
pub fn Education(
    educations: ReadSignal<Vec<Education>>,
    #[prop(optional)] on_delete: Option<Callback<usize>>,
    #[prop(optional)] on_edit: Option<Callback<usize>>,
    is_edit: bool
) -> impl IntoView {
    {
        move ||
            educations
                .get()
                .into_iter()
                .enumerate()
                .map(|(index, education)| {
                    let url = if education.institute_logo_url.is_empty() {
                        "https://cdn-icons-png.flaticon.com/512/4729/4729436.png".to_string()
                    } else {
                        education.institute_logo_url.clone()
                    };

                    view! {
                        <div class="educationContainer">
                            <span class="experienceRow">
                                <span class="educationRowFirstItem">
                                    <img src=url alt="Institute Icon" />
                                    <div class="experienceRowFirstItemText">
                                        <h3><b>Institute Name</b>: {education.institute_name}</h3>
                                        <h3><b>Degree</b>: {education.degree}</h3>
                                        <h3> <b>Major</b>:{education.major} </h3>
                                        <h3><b>Institute Address</b>: {education.institute_address}</h3> 
                                        <div class="rowItem">
                                        <h3 class="experienceh3" style="margin-right:10px;" > <b>GPA</b>:{education.gpa} </h3>
                                        <h3 class="experienceh3"> <b>Graduated Year</b>:{education.graduated_year} </h3>
                                       </div>
                                    </div>
                                </span>
                                { view! {
                                    <>
                                        {if is_edit {
                                            view! {
                                                <div class="iconRow" >
                                                  
                                                    <button
                                                        class="editButton iconMargin"
                                                        type="button" 
                                                        on:click=move |_| {
                                                            if let Some(ref callback) = on_edit {
                                                                leptos::Callable::call(callback, index);
                                                            }
                                                        }
                                                    >
                                                        <Icon icon={i::BiEditRegular} />
                                                    </button>
                                                    <button
                                                        class="deleteButton"
                                                        type="button" 
                                                        on:click=move |_| {
                                                            if let Some(ref callback) = on_delete {
                                                                leptos::Callable::call(callback, index);
                                                            }
                                                        }
                                                    >
                                                        <Icon icon={i::BsTrash} />
                                                    </button>
                                                </div>
                                            }
                                        } else {
                                            view! { <div></div> }
                                        }}
                                    </>
                                } }
                            </span>
                        
                        </div>
                    }
                })
                .collect::<Vec<_>>()
    }
}
