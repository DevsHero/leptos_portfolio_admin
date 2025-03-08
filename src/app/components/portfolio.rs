use leptos::*;
use crate::app::{ components::ImageSlider, models::portfolio::Portfolio };
use leptos_icons::Icon;
use icondata as i;
#[component]
pub fn Portfolio(
    portfolios: ReadSignal<Vec<Portfolio>>,
    on_delete: Option<Callback<usize>>, // made optional
    use_delete: bool
) -> impl IntoView {
    {
        move ||
            portfolios
                .get()
                .into_iter()
                .enumerate()
                .map(|(index, portfolio)| {
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
                // <div class="projectIcons">{icns}</div>
       
                </div>
   <div class="portfolioSlide">  <ImageSlider images=portfolio.screenshots_url/></div>
       </div>
       <div class="editContactRow">
       <div class="stackRow">
       <b  >Stack:</b> {let stacks = portfolio.stacks.clone();
           move || stacks.iter().enumerate().map(|(index, stack)| {
               view! { <p  >{index +1}.{stack} </p> }
           }).collect::<Vec<_>>()}
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
           </div>
           </div> 
   </div>
         }
                })
                .collect::<Vec<_>>()
    }
}
