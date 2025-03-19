use leptos::*;
use crate::app::{ models::portfolio::Experience, utils::convert_date_format };
use leptos_icons::Icon;
use icondata as i;
#[component]
pub fn Experience(
    experiences: ReadSignal<Vec<Experience>>,
    #[prop(optional)] on_delete: Option<Callback<usize>>,
    #[prop(optional)] on_edit: Option<Callback<usize>>,
    is_edit: bool
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
                    let aLink = if experience.company_url.is_empty() {
                        view! { <div></div> }
                    } else {
                        view! { <div style="margin-left: 5px; color:blue;"> <a href=experience.company_url target="_blank" >  
                        <Icon  icon={i::TbWorldWww} /> 
                        </a></div> }
                    };
                    view! {
               <div class="experienceContainer">
                <span class="experienceRow">
              
                <span class="experienceRowFirstItem">
                    <img src=url alt="Company Icon" />
                    <div class= "experienceRowFirstItemText" >
                    <h4 class="experienceH4" ><b>Company</b>: {experience.company_name}</h4> 
                    <h4 class="experienceH4" ><b>Position</b>: {experience.position_name}</h4>
                   
                    <h4 class="experienceH4" ><b>Address</b>: {experience.company_address}</h4>
                   <div class="rowItem"> 
                   <h4 class="experienceH4" ><b>Period</b>: 
                   {convert_date_format(&experience.start_date) } - {convert_date_format(&experience.end_date) }</h4> 
                  {aLink}
                    </div>   
                     </div>
                    </span>  
                    { view! {
                            <>
                                {if is_edit {
                                    view! {
                                        <div class="inputArrayRow">
                                        <button
                                            class="editButton"
                                            style="margin-right:10px;"
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
                        }  }
                </span>    
                <div class="descriptions" inner_html=experience.describe></div>
       </div>
         }
                })
                .collect::<Vec<_>>()
    }
}
