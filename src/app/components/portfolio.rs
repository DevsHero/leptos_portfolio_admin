use leptos::*;
use crate::app::{ components::ImageSlider, models::portfolio::Portfolio };
use leptos_icons::Icon;
use icondata as i;
#[component]
pub fn Portfolio(
    portfolios: ReadSignal<Vec<Portfolio>>,
    #[prop(optional)] on_delete: Option<Callback<usize>>,
    #[prop(optional)] on_edit: Option<Callback<usize>>,
    #[prop(optional)] set_portfolios: Option<WriteSignal<Vec<Portfolio>>>,
    #[prop(optional)] set_is_update: Option<WriteSignal<bool>>,
    is_edit: bool
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
            portfolios
                .get()
                .into_iter()
                .enumerate()
                .map(|(index, portfolio)| {
                    let edit_menu = {
                        if is_edit {
                            view! {
                           <div class="iconRow">
                           <button
                           type="button"
                           class="moveButton"
                           on:click=move |_| {
                               if let Some(ref set_portfolios) = set_portfolios {
                                   set_portfolios.update(|list| {
                                       if index > 0 {
                                           list.swap(index, index - 1);
                                           list[index].index = index as u8;
                                           list[index - 1].index = (index - 1) as u8;
                                           set_is_update.unwrap()(true);
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
                                   set_portfolios.update(|list| {
                                       if index < list.len() - 1 {
                                           list.swap(index, index + 1);
                                           list[index].index = index as u8;
                                           list[index + 1].index = (index + 1) as u8;
                                           set_is_update.unwrap()(true);
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
                                       leptos::Callable::call(callback, index);
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
                        "https://cdn-icons-png.flaticon.com/512/7867/7867852.png".to_string()
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
                             <h3><b>Opensource</b>: {if portfolio.is_private {"No"} else {"Yes"} }</h3> 
                             </div>
                         </div>
                       
                         <div class="portfolioDescriptions" inner_html=portfolio.portfolio_detail></div>    
                         </div>
                       <div  >  
                    { if !is_mobile.get() { edit_menu_clone.clone()} else{view! {<div></div>}}}
                <ImageSlider images=portfolio.screenshots_url/>
                </div>
                </div>
                <div class="editContactRow">
                <div class="stackRow">
                {if portfolio.stacks.len() > 1 {
                 view!{
                  
                    <b  >Stack:</b> {let stacks = portfolio.stacks.clone();
                        move || stacks.iter().enumerate().map(|(index, stack)| {
                            view! { <p style="margin-left:5px"  >{index +1}.{stack} </p> }
                        }).collect::<Vec<_>>()}
                }} else { 
                 view!{
                    <b></b> <></>
                }} }
              
                       
                 
                    </div>
                    </div> 
            </div>}
                })
                .collect::<Vec<_>>()
    }
}
