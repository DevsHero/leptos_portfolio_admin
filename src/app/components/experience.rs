use leptos::{ either::Either, prelude::* };
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
    (
        view! {
        <For
        each=move || experiences.get().into_iter().enumerate() 
        key=|(index, _experience)| *index
        children=move |(index, experience)| {
                    let url = if experience.company_logo_url.is_empty() {
                        "https://cdn-icons-png.flaticon.com/512/6214/6214253.png".to_string()
                    } else {
                        experience.company_logo_url.clone()
                    };
                  
                    view! {
                        <div class="experienceContainer">
                         
                                <span class="experienceRowFirstItem"   >
                                    <img src=url alt="Company Icon" />
                                    <div class="experienceRowFirstItemText">
                                    <div class="rowItem">
                                        <h3 ><b>Company</b>: {experience.company_name}</h3> </div>
                                        <h3  ><b>Position</b>: {experience.position_name}</h3>
                                        <h3  ><b>Address</b>: {experience.company_address}</h3>
                                        <div class="rowItem">
                                            <h3 class="experienceh3" >
                                                <b>Period</b>:
                                                {convert_date_format(&experience.start_date)} - {convert_date_format(&experience.end_date)}
                                            </h3>
                                         {if experience.company_url.is_empty() {
                                            None
                                        } else {
                                           Some(
                                                view! {
                                                    <a style="margin-left: 5px; color:blue;" href=experience.company_url target="_blank">
                                                        <Icon icon={i::TbWorldWww} />
                                                    </a>
                                            }
                                            )
                                        }}
                                        </div>
                                    </div>
                                    {if is_edit {
                                        Either::Left(      view! {
                                            <div class="iconRow">
                                                <button
                                                    class="editButton iconMargin"
                                                    type="button" 
                                                    on:click=move |_| {
                                                        if let Some(ref callback) = on_edit {
                                                            callback.run( index);
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
                                                            callback.run( index);
                                                        }
                                                    }
                                                >
                                                    <Icon icon={i::BsTrash} />
                                                </button>
                                            </div>
                                        } )
                                    } else {
                                        Either::Right(())
                                    }}
                                </span>
                             
                            <div class="descriptions" inner_html=experience.describe></div>
                        </div>
                    }  
                } 
              
                />
    }
    ).into_any()
}
