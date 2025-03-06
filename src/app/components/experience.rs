use leptos::*;
use crate::app::{ models::portfolio::Experience, utils::convert_date_format };
use leptos_icons::Icon;
use icondata as i;
#[component]
pub fn Experience(
    experiences: ReadSignal<Vec<Experience>>,
    on_delete: Option<Callback<usize>>, // made optional
    is_page: bool,
    use_delete: bool
) -> impl IntoView {
    view! {
        {
            move ||
                experiences
                    .get()
                    .into_iter()
                    .enumerate()
                    .map(|(index, experience)| {
                        view! {
        
               <div class= if is_page {"experiencePageContainer"} else {"experience-container"}>
                <span class="experienceRow">
                <a href=experience.company_url target="_blank" >
                <span class="experienceRowFirstItem">
                    <img src=experience.company_logo_url alt="Company Icon" />
                    <div class="experienceRowFirstItemText">
                    <h4><b>Company</b>: {experience.company_name}</h4> <h4><b>Position</b>: {experience.position_name}</h4>
                    <p>{convert_date_format(&experience.start_date) } - {convert_date_format(&experience.end_date) }</p>
                    </div>
                    </span>
                    </a>
                    <div class="experienceRowFirstItemText">
                    // <b><span class="experienceNumber">{(index + 1).to_string()} </span></b>
                    {
                        // Wrap the conditional in a fragment so both branches return the same type.
                        view! {
                            <>
                                {if use_delete {
                                    view! {
                                        <div>
                                            <button
                                                class="deleteButton"
                                                on:click=move |_| {
                                                    if let Some(ref callback) = on_delete {
                                                        callback.call(index);
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
                        }
                    }
                    </div>
                </span>
                
                <div class="descriptions" inner_html=experience.describe></div>
                // <div class="projectIcons">{icns}</div>
       
       </div>
         }
                    })
                    .collect::<Vec<_>>()
        }
    }
}
