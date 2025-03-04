use leptos::*;
use crate::app::{ components::ImageSlider, models::portfolio::Portfolio };
#[component]
pub fn Portfolio(
    portfolios: ReadSignal<Vec<Portfolio>>,
    on_delete: Option<Callback<usize>>, // made optional
    is_page: bool,
    use_delete: bool
) -> impl IntoView {
    view! {
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
                    <div class="experienceRowFirstItemText"> 
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
                                                        callback.call(index);
                                                    }
                                                }
                                            >
                                            <b> "×"</b>
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
            
                
                <div class="portfolioDescriptions" inner_html=portfolio.portfolio_detail></div>
                // <div class="projectIcons">{icns}</div>
       
                </div>
   <div class="portfolioSlide">  <ImageSlider images=portfolio.screenshots_url/></div>
       </div>
       <div class="stackRow">
       <b class="pr-4">Stack:</b> {let stacks = portfolio.stacks.clone();
           move || stacks.iter().enumerate().map(|(index, stack)| {
               view! { <p class="pr-4">{index +1}.{stack} </p> }
           }).collect::<Vec<_>>()}
           </div>
   </div>
         }
                    })
                    .collect::<Vec<_>>()
        }
    }
}
