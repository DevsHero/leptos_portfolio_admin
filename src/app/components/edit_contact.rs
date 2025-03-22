use crate::app::utils::get_icon_by_name;
use leptos::{ either::Either, prelude::* };
use leptos_icons::Icon;
use icondata as i;
use crate::app::models::portfolio::Contact;
#[component]
pub fn EditContacts(
    contacts: ReadSignal<Vec<Contact>>,
    #[prop(optional)] on_delete: Option<Callback<usize>>,
    #[prop(optional)] on_edit: Option<Callback<usize>>,
    is_edit: bool
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
                    <div style="  display: flex; flex-direction: row;   align-items: center;">
                    <p style="margin-right: 5px;">Icon : </p>
                <Icon style="margin-right: 5px;" icon={get_icon.unwrap_or(i::BiErrorSolid)} />
                <p>{contact.contact_icon }</p>
                </div>
                {
      
                    view! {
                        <>
                            {if is_edit {
                                Either::Left(     view! {
                                    <div class="inputArrayRow">
                                        <button
                                        type="button" 
                                            class="editButton"
                                            style="margin-right:10px;"
                                            on:click=move |_| {
                                                if let Some(ref callback) = on_edit {
                                                    (callback, index);
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
                                                (callback, index);
                                            }
                                        }
                                    >
                                    <Icon icon={i::BsTrash} />
                                    </button>
                                    </div>
                                }
                 ) } else {
                                Either::Right(())
                            }}
                        </>
                    }
                }
                </div>
                <p>Title : {contact.contact_title} </p>    
                <p>Value : {contact.contact_value} </p>
                <p>Use Link : {contact.use_link} </p>        
         </div>  }
                })
                .collect_view()
    }
}
