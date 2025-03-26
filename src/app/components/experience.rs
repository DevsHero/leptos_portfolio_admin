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
                        view! {
                            <div style="margin-left: 5px; color:blue;">
                                <a href=experience.company_url target="_blank">
                                    <Icon icon={i::TbWorldWww} />
                                </a>
                            </div>
                        }
                    };
                    view! {
                        <div class="experienceContainer">
                            <span class="experienceRow">
                                <span class="experienceRowFirstItem">
                                    <img src=url alt="Company Icon" />
                                    <div class="experienceRowFirstItemText">
                                        <h3 ><b>Company</b>: {experience.company_name}</h3>
                                        <h3  ><b>Position</b>: {experience.position_name}</h3>
                                        <h3  ><b>Address</b>: {experience.company_address}</h3>
                                        <div class="rowItem">
                                            <h3 class="experienceh3" >
                                                <b>Period</b>:
                                                {convert_date_format(&experience.start_date)} - {convert_date_format(&experience.end_date)}
                                            </h3>
                                            {aLink}
                                        </div>
                                    </div>
                                </span>
                                { view! {
                                    <>
                                        {if is_edit {
                                            view! {
                                                <div class="iconRow">
                                                  
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
                            <div class="descriptions" inner_html=experience.describe></div>
                        </div>
                    }
                })
                .collect::<Vec<_>>()
    }
}
