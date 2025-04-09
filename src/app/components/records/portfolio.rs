use leptos::*;
use crate::app::{
    components::utils::ImageSlider,
    constants::constant::PORTFOLIO_NONE_ICON,
    models::profile::Portfolio,
};
use leptos_icons::Icon;
use icondata as i;
#[component]
pub fn PortfolioRecords(
    portfolios: ReadSignal<Vec<Portfolio>>,
    #[prop(optional)] on_delete: Option<Callback<usize>>,
    #[prop(optional)] on_edit: Option<Callback<usize>>,
    #[prop(optional)] set_portfolios: Option<WriteSignal<Vec<Portfolio>>>,
    #[prop(optional)] set_is_update: Option<WriteSignal<bool>>,
    is_edit: bool
) -> impl IntoView {
    let (is_mobile, set_is_mobile) = create_signal(false);
    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(width) = window.inner_width().map(|w| w.as_f64().unwrap_or(0.0)) {
                set_is_mobile(width < 768.0);
            }
        }
    });
    view! {
        <For
        each=move ||portfolios.get() 
        key=|portfolio| portfolio.uuid.clone() 
                children=move |portfolio: Portfolio| {
                    let uuid_up = portfolio.uuid.clone();
                    let uuid_down = portfolio.uuid.clone();
                    let uuid_edit = portfolio.uuid.clone();
                    let uuid_delete = portfolio.uuid.clone();
                    let edit_menu = {
                        if is_edit {
                            view! {
                           <div class="iconRow" style="align-items:  center;">
                           <button
                           type="button"
                           class="moveButton"
                           on:click=move |_| {
                            if let Some(ref set_portfolios) = set_portfolios {
                                let current_index = portfolios.get()
                                    .iter()
                                    .position(|p| p.uuid == uuid_up)
                                    .expect("Portfolio not found");
                                set_portfolios.update(|list| {
                                    if current_index > 0 {
                                        list.swap(current_index, current_index - 1);
                                        list[current_index].index = current_index as u8;
                                        list[current_index - 1].index = (current_index - 1) as u8;
                                        if let Some(set_is_update) = set_is_update {
                                            set_is_update(true);
                                        }
                                    }
                                });
                            }
                        }
                    >
                       
                           <Icon icon={i::BiChevronUpCircleRegular} />
                       </button>
                       <button
                       type="button"
                           class="moveButton"
                           on:click=move |_| {
                            if let Some(ref set_portfolios) = set_portfolios {
                                let current_index = portfolios.get()
                                    .iter()
                                    .position(|p| p.uuid == uuid_down)
                                    .expect("Portfolio not found");
                                set_portfolios.update(|list| {
                                    if current_index < list.len() - 1 {
                                        list.swap(current_index, current_index + 1);
                                        list[current_index].index = current_index as u8;
                                        list[current_index + 1].index = (current_index + 1) as u8;
                                        if let Some(set_is_update) = set_is_update {
                                            set_is_update(true);
                                        }
                                    }
                                });
                            }
                        }
                    >
                           <Icon icon={i::BiChevronDownCircleRegular} />
                       </button>
                           <button
                           type="button"
                               class="editButton"
                               on:click=move |_| {
                                if let Some(ref callback) = on_edit {
                                    let current_index = portfolios.get()
                                        .iter()
                                        .position(|p| p.uuid == uuid_edit)
                                        .expect("Portfolio not found");
                                    leptos::Callable::call(callback, current_index);
                                }
                            }
                        >
                               <Icon icon={i::BiEditRegular} />
                           </button>
                           <button
                           type="button"
                               class="deleteButton"
                               on:click=move |_| {
                                if let Some(ref callback) = on_delete {
                                    let current_index = portfolios.get()
                                        .iter()
                                        .position(|p| p.uuid == uuid_delete)
                                        .expect("Portfolio not found");
                                    leptos::Callable::call(callback, current_index);
                                }
                            }
                        >
                               <Icon icon={i::BsTrash} />
                           </button>
                       </div>
                        }
                        } else {
                            view! { <div></div> }
                        }
                    };
                    let edit_menu_clone = edit_menu.clone();
                    let aLink: HtmlElement<html::Div> = if portfolio.portfolio_link.is_empty() {
                        view! { <div></div> }
                    } else {
                        view! { <div style=" margin-top:2px;  align-items: end; color:blue;"> <a href=portfolio.portfolio_link target="_blank" >  
                        <Icon  icon={i::TbWorldWww} /> 
                        </a></div> }
                    };

                    let url = if portfolio.portfolio_icon_url.is_empty() {
                        PORTFOLIO_NONE_ICON.to_string()
                    } else {
                        portfolio.portfolio_icon_url.clone()
                    };
                    view! {
                       <div class="portfolioContainer">     
                        <div class="portfolioRow">    
                        <div class="portfolioColumn">
                         <div class="portfolioHeader">
                             <img src=url alt="Portfolio Icon" />
                             <div class="experienceRowFirstItemText">
                             { if is_mobile.get() { edit_menu.clone()} else{view! {<div></div>}}}
                             <h3><b>Name</b>: {portfolio.portfolio_name}</h3>  
                             <h3><b>Opensource</b>: {if portfolio.is_opensource {"Yes"} else {"No"} } {aLink}</h3> 
                             </div>
                             
                         </div>
                       
                         <div class="portfolioDescriptions" inner_html=portfolio.portfolio_detail></div>    
                         </div>
                       <div  >  
                    { if !is_mobile.get() { edit_menu_clone.clone()} else{view! {<div></div>}}}
                <ImageSlider images=portfolio.screenshots_url/>
                </div>
                </div>
              
                <div class="stackRow">
                {if portfolio.stacks.len() > 1 {
                Some( view!{
                    <b  >Stack:</b> {let stacks = portfolio.stacks.clone();
                        move || stacks.iter().enumerate().map(|(index, stack)| {
                            view! { <p style="margin-left:5px"  >{index +1}.{stack} </p> }
                        }).collect::<Vec<_>>()}
                })
                } else { 
                 None} }
                    </div>    
            </div>}
                }
                /> 
    }
}
