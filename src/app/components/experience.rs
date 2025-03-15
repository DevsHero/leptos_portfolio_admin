use leptos::*;
use crate::app::{ models::portfolio::Experience, utils::convert_date_format };
use leptos_icons::Icon;
use icondata as i;
#[component]
pub fn Experience(
    experiences: ReadSignal<Vec<Experience>>,
    on_delete: Option<Callback<usize>>,

    use_delete: bool
) -> impl IntoView {
    {
        move ||
            experiences
                .get()
                .into_iter()
                .enumerate()
                .map(|(index, experience)| {
                    let url = if experience.company_logo_url.is_empty() {
                        "https://cdn-icons-png.flaticon.com/512/6214/6214253.png".to_string()
                    } else {
                        experience.company_logo_url.clone()
                    };
                    view! {
        
               <div class="experienceContainer">
                <span class="experienceRow">
                <a href=experience.company_url target="_blank" >
                <span class="experienceRowFirstItem">
                    <img src=url alt="Company Icon" />
                    <div class= "experienceRowFirstItemText" >
                    <h4 class="experienceH4" ><b>Company</b>: {experience.company_name}</h4> 
                    <h4 class="experienceH4" ><b>Position</b>: {experience.position_name}</h4>
                    <p>{convert_date_format(&experience.start_date) } - {convert_date_format(&experience.end_date) }</p>
                    </div>
                    </span>
                    </a>
              
                    {
              
                        view! {
                            <>
                                {if use_delete {
                                    view! {
                                        <div>
                                            <button
                                                class="deleteButton"
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
                        }
                    }
               
                </span>
                
                <div class="descriptions" inner_html=experience.describe></div>
 
       
       </div>
         }
                })
                .collect::<Vec<_>>()
    }
}
