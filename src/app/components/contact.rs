use leptos::*;
use leptos_icons::Icon;
use icondata as i;

use crate::app::{ models::portfolio::Contact, utils::get_icon_by_name };
use crate::app::components::Dialog;
#[component]
pub fn HomeContacts(contacts: Vec<Contact>) -> impl IntoView {
    view! {
        <div class="contacts">
 
    
        {
       
            contacts
                .into_iter()
                .map(|contact| {
                    let get_icon = get_icon_by_name(&contact.contact_icon);
                    if contact.use_link {
                        view! {
                            <>
                        <a  href=contact.contact_value 
                            target="_blank"    >
                        <div class="contactIcon">
                        <Icon icon={get_icon.unwrap_or(i::BiErrorSolid)} />
                        </div>
                        </a>  
                        </>
                    }
                    } else {
                        view! {
                            <>  
                    <Dialog children_only=false detail=contact.contact_value title=contact.contact_title.unwrap_or("".to_string())   >
                    <div class="contactIcon">
                    <Icon icon={get_icon.unwrap_or(i::BiErrorSolid)} />
                    </div>
                   </Dialog>
             </>
                    
                    }
                    }
                })
                .collect::<Vec<_>>()
              }   </div>
    }
}
