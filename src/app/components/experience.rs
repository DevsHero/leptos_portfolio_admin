use leptos::*;
use crate::app::{ models::portfolio::Experience, utils::convert_date_format };
use leptos_icons::Icon;
use icondata as i;
#[component]
pub fn Experience(
    experiences: ReadSignal<Vec<Experience>>,
    on_delete: Option<Callback<usize>>,
    is_page: bool,
    use_delete: bool
) -> impl IntoView {
    {
        let (is_mobile, set_is_mobile) = create_signal(false);
        create_effect(move |_| {
            if let Some(window) = web_sys::window() {
                if let Ok(width) = window.inner_width().map(|w| w.as_f64().unwrap_or(0.0)) {
                    // Here 768 is an example breakpoint; adjust as needed.
                    set_is_mobile(width < 768.0);
                }
            }
        });
        move ||
            experiences
                .get()
                .into_iter()
                .enumerate()
                .map(|(index, experience)| {
                    view! {
        
               <div class= if is_mobile.get() {"experienceMobileContainer"} else if is_page {"experiencePageContainer"} else {"experience-container"}>
                <span class="experienceRow">
                <a href=experience.company_url target="_blank" >
                <span class="experienceRowFirstItem">
                    <img src=experience.company_logo_url alt="Company Icon" />
                    <div class= "experienceRowFirstItemText" >
                    <h4 style=if is_mobile.get(){"font-size: 13px;"} else{""}><b>Company</b>: {experience.company_name}</h4> 
                    <h4 style=if is_mobile.get(){"font-size: 13px;"} else{""}><b>Position</b>: {experience.position_name}</h4>
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
