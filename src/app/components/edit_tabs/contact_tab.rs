use crate::app::components::{
    show_error_toast,
    show_success_toast,
    CheckBox,
    EditContacts,
    IconDropdown,
    InputField,
    RenderTab,
};

use crate::app::models::Contact;
use leptos::*;

#[component]
pub fn EditContactTab(
    contacts: ReadSignal<Vec<Contact>>,
    set_contacts: WriteSignal<Vec<Contact>>,
    set_is_update_contact: WriteSignal<bool>,
    select_tab: ReadSignal<i32>
) -> impl IntoView {
    let (contact_value, set_contact_value) = create_signal(String::new());
    let (contact_icon, set_contact_icon) = create_signal(String::new());
    let (contact_title, set_contact_title) = create_signal(String::new());
    let (use_link, set_use_link) = create_signal(false);
    let (validate_contact, set_validate_contact) = create_signal(false);

    let add_contact = move |_| {
        set_validate_contact.update(|v| {
            *v = !*v;
        });
        let form_valid =
            !contact_value.get().trim().is_empty() && !contact_icon.get().trim().is_empty();
        if form_valid {
            let new_contact = Contact {
                contact_icon: contact_icon.get(),
                contact_value: contact_value.get(),
                contact_title: Some(contact_title.get()),
                use_link: use_link.get(),
            };
            set_contacts.update(|contact| contact.push(new_contact));
            set_validate_contact.set(false);
            set_contact_icon.set(String::new());
            set_contact_value.set(String::new());
            set_contact_title.set(String::new());
            set_use_link.set(false);
            set_is_update_contact(true);
            show_success_toast("Add Contact Success", "Contact Added.");
        } else {
            show_error_toast("Add Contact Failed", "Missing required field.");
        }
    };

    let delete_contact = move |index: usize| {
        set_contacts.update(|contacts| {
            contacts.remove(index);
        });
        set_is_update_contact(true)
    };

    let edit_contact = move |index: usize| {
        let list = contacts.get();
        if
            let Some(contact) = list
                .iter()
                .enumerate()
                .find(|(i, _)| *i == index)
        {
            let contact = contact.1.clone();
            set_contact_title.set(contact.contact_title.unwrap_or(String::from("")));
            set_contact_value.set(contact.contact_value);
            set_contact_icon.set(contact.contact_icon);
            set_use_link.set(contact.use_link);
            delete_contact(index);
        }
    };
    view! {
        <RenderTab  no=5 active_page=select_tab>
                  <Show when=move || select_tab() == 5>
                  <Suspense fallback=move || view! { <p>"LoadingIntro..."</p> }>
                  <div class="editContainer">
                  <h1>"Edit Contact"</h1>
                  {move ||view! { <CheckBox id="use_link"  label= "Use link (disable dialog)" set_value=set_use_link  get_value=use_link />}}
                  <IconDropdown validation=validate_contact label="Contact Icon"  get_value=contact_icon  set_value=set_contact_icon require=true  / >
                  {move || {if !use_link.get() {
                      view! {
                          <div>
                          <InputField input_type="text" id="contact_title" label="Contact Title (Show in dialog)" set_value=set_contact_title  get_value=contact_title require=true />
                          </div>
                      }
                  } else {
                      view! { <div></div> }
                  }}}
                  <InputField validation=validate_contact input_type="text" id="contact_value" label="Contact Value" set_value=set_contact_value  get_value=contact_value require=true />  
                  <button
                          type="button"
                          class="addButton"
                          on:click=add_contact >
                          "Add Contact"
                  </button>
                      <EditContacts
                      contacts=contacts  
                      on_delete=Callback::new(move |index| delete_contact(index))
                      on_edit=Callback::new(move |index| edit_contact(index))
                      is_edit=true/ >
                    </div>
                    </Suspense>
                    </Show>
                  </RenderTab>
    }
}
