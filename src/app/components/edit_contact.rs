use crate::app::utils::get_icon_by_name;
use leptos::*;
use leptos_icons::Icon;
use icondata as i;
use crate::app::models::portfolio::Contact;
#[component]
pub fn EditContacts(
    contacts: ReadSignal<Vec<Contact>>,
    on_delete: Option<Callback<usize>>, // made optional

    use_delete: bool
) -> impl IntoView {
    {
        move ||
            contacts
                .get()
                .into_iter()
                .enumerate()
                .map(|(index, contact)| {
                    let get_icon = get_icon_by_name(&contact.contact_icon);
                    view! {
                    <div class="editContactContainer">
            
                    <div  class="editContactRow" >
                    <div style="  display: flex;
  flex-direction: row;   align-items: center;">
                    <p style="margin-right: 5px;">Icon : </p>
                <Icon style="margin-right: 5px;" icon={get_icon.unwrap_or(i::BiErrorSolid)} />
                <p>{contact.contact_icon }</p>
                </div>
                {
                    // Wrap the conditional in a fragment so both branches return the same type.
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
                <p>Value : {contact.contact_value} </p>
                <p>Use Href : {contact.is_href} </p>
           
                        
              
                        </div>  }
                })
                .collect::<Vec<_>>()
    }
}
