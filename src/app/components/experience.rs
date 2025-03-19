use leptos::*;
use leptos_use::{ use_draggable, use_droppable };
use crate::app::{ models::portfolio::Experience, utils::convert_date_format };
use leptos_icons::Icon;
use icondata as i;

#[component]
pub fn Experience(
    experiences: ReadSignal<Vec<Experience>>,
    #[prop(optional)] on_delete: Option<Callback<usize>>,
    #[prop(optional)] on_edit: Option<Callback<usize>>,
    #[prop(optional)] on_reorder: Option<Callback<(usize, usize)>>,
    is_edit: bool
) -> impl IntoView {
    // Draggable/droppable states
    let (dragging_index, set_dragging_index) = create_signal::<Option<usize>>(None);
    let (hover_index, set_hover_index) = create_signal::<Option<usize>>(None);

    // Handle drag and drop logic
    let handle_drop = move |from: usize, to: usize| {
        if let Some(callback) = on_reorder {
            callback.call((from, to));
        }
    };

    view! {
        <div class="experience-list">
            {move || experiences.get().into_iter().enumerate().map(|(index, experience)| {
                let url = if experience.company_logo_url.is_empty() {
                    "https://cdn-icons-png.flaticon.com/512/6214/6214253.png".to_string()
                } else {
                    experience.company_logo_url.clone()
                };

                let a_link = if experience.company_url.is_empty() {
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

                // Drag and drop functionality
                let (drag_el, drag_state) = use_draggable();
                let (drop_el, drop_state) = use_droppable();

                let _ = drag_state.set_handle(Some(drop_el.get_untracked()));
                
                create_effect(move |_| {
                    if drag_state.dragging() {
                        set_dragging_index.set(Some(index));
                    } else if let Some(dragging) = dragging_index.get() {
                        if let Some(hover) = hover_index.get() {
                            handle_drop(dragging, hover);
                        }
                        set_dragging_index.set(None);
                        set_hover_index.set(None);
                    }
                });

                create_effect(move |_| {
                    if drop_state.is_over() {
                        set_hover_index.set(Some(index));
                    }
                });

                view! {
                    <div
                        class="experienceContainer"
                        class:dragging=move || dragging_index.get() == Some(index)
                        class:drop-target=move || hover_index.get() == Some(index)
                        node_ref=drop_el
                    >
                        <div class="experienceRow" node_ref=drag_el>
                            <span class="experienceRowFirstItem">
                                <img src=url alt="Company Icon" />
                                <div class="experienceRowFirstItemText">
                                    <h4 class="experienceH4"><b>Company</b>: {experience.company_name}</h4> 
                                    <h4 class="experienceH4"><b>Position</b>: {experience.position_name}</h4>
                                    <h4 class="experienceH4"><b>Address</b>: {experience.company_address}</h4>
                                    <div class="rowItem"> 
                                        <h4 class="experienceH4">
                                            <b>Period</b>: 
                                            {convert_date_format(&experience.start_date)} - {convert_date_format(&experience.end_date)}
                                        </h4> 
                                        {a_link}
                                    </div>
                                </div>
                            </span>

                            {if is_edit {
                                view! {
                                    <div class="inputArrayRow">
                                        <span class="drag-handle">
                                            <Icon icon={i::FiGripVertical} />
                                        </span>
                                        <button
                                            class="editButton"
                                            style="margin-right:10px;"
                                            on:click=move |_| {
                                                if let Some(ref callback) = on_edit {
                                                    callback.call(index);
                                                }
                                            }
                                        >
                                            <Icon icon={i::BiEditRegular} />
                                        </button>
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
                        </div>
                        <div class="descriptions" inner_html=experience.describe></div>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
