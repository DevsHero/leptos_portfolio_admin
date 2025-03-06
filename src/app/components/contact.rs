use leptos::*;
use leptos_icons::Icon;
use icondata as i;

use crate::app::{ models::portfolio::Contact, utils::get_icon_by_name };

#[component]
pub fn Contacts(contacts: Vec<Contact>) -> impl IntoView {
    view! {
        <div class="socialButtons">
        {
       
            contacts
                .into_iter()
                .map(|contact| {
                    let maybe_icon = get_icon_by_name(&contact.contact_icon);
                    if contact.is_href {
                        view! {
                        <a 
                            href=contact.contact_value 
                            target="_blank" 
                          
                        >
                      
                        <Icon icon={maybe_icon.unwrap_or(i::BiErrorSolid)} />
                
                        </a>
                    }
                    } else {
                        view! {
                        <a 
                       class="aLinkRow"
                        href=""
                        target="_blank" 
                        style="pointer-events: none; opacity: 0.6;"
                      
                    >
                 
                    <Icon icon={maybe_icon.unwrap_or(i::BiErrorSolid)} />
                
                    <p> {contact.contact_value} </p>
                    </a>
                      
                    }
                    }
                })
                .collect::<Vec<_>>()
              }   </div>
    }
}
