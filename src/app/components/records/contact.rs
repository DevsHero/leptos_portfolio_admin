use leptos::*;
use leptos_icons::Icon;
use icondata as i;

use crate::app::components::layouts::Dialog;
use crate::app::components::utils::PdfExportButton;
use crate::app::models::Profile;
use crate::app::utils::utils::get_icon_by_name;

#[component]
pub fn ContactRecords(is_ready: ReadSignal<bool>, profile: Profile) -> impl IntoView {
    let pdf = profile.pdf.clone();
    let profile_clone = profile.clone();
    view! {
        
       
 <div class=move || if !is_ready.get() { "loadingContact " } else { "contacts" }>
 {move || if pdf.use_pdf  {
    Some(view! { <PdfExportButton profile=profile_clone.clone() />  })
} else {
    None
}}
        {
       
            profile.contacts.unwrap()
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
