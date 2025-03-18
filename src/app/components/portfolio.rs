use leptos::*;
use crate::app::{ components::ImageSlider, models::portfolio::Portfolio };
use leptos_icons::Icon;
use icondata as i;
#[component]
pub fn Portfolio(
    portfolios: ReadSignal<Vec<Portfolio>>,
    #[prop(optional)] on_delete: Option<Callback<usize>>,
    #[prop(optional)] on_edit: Option<Callback<usize>>,
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
                    view! {
                        {
                            // select css design by device
                            if is_mobile.get() {
                                view! {
                        <div class="portfolioContainer"  >     
                      
                        {
          
                            view! {
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
                            }
                        }
                         <a href=portfolio.portfolio_link target="_blank" >
                         <div class="portfolioHeader">
                             <img src=portfolio.portfolio_icon_url alt="Project Icon" />
                             <div class="experienceRowFirstItemText">
                             <h4 style={"font-size: 13px"}><b>Name</b>: {portfolio.portfolio_name}</h4>  
                             <h4 style={"font-size: 13px"}><b>Opensource</b>: {if portfolio.is_private {"No"} else {"Yes"} }</h4> 
                             </div>
                         </div>
                         </a>
                         <div class="portfolioDescriptions"   inner_html=portfolio.portfolio_detail ></div>    
                         <ImageSlider images=portfolio.screenshots_url/>
               
         
                <div class="stackRow">
                <b  >Stack:</b> {let stacks = portfolio.stacks.clone();
                    move || stacks.iter().enumerate().map(|(index, stack)| {
                        view! { <p style="margin-left:5px" >{index +1}.{stack} </p> }
                    }).collect::<Vec<_>>()}
                     
                
                    </div> 
            </div>
                        }
                            } else {
                                view! {
                       <div class="portfolioContainer">     
                        <div class="portfolioRow">    
                        <div class="portfolioColumn">
                         <a href=portfolio.portfolio_link target="_blank" >
                         <div class="portfolioHeader">
                             <img src=portfolio.portfolio_icon_url alt="Project Icon" />
                             <div class="experienceRowFirstItemText">
                             <h4><b>Name</b>: {portfolio.portfolio_name}</h4>  
                             <h4><b>Opensource</b>: {if portfolio.is_private {"No"} else {"Yes"} }</h4> 
                             </div>
                         </div>
                         </a>
                         <div class="portfolioDescriptions" inner_html=portfolio.portfolio_detail></div>    
                         </div>
                         
            <ImageSlider images=portfolio.screenshots_url/>
                </div>
                <div class="editContactRow">
                <div class="stackRow">
                <b  >Stack:</b> {let stacks = portfolio.stacks.clone();
                    move || stacks.iter().enumerate().map(|(index, stack)| {
                        view! { <p style="margin-left:5px"  >{index +1}.{stack} </p> }
                    }).collect::<Vec<_>>()}
                       {
          
                     view! {
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
                     }
                 }
                    </div>
                    </div> 
            </div>}
                            }
                        }
                    }
                })
                .collect::<Vec<_>>()
    }
}
