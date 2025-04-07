use leptos::*;
use crate::app::{
    components::{ show_info_toast, show_error_toast, InputField, Topbar },
    server::api::verify_password_api,
};

#[component]
pub fn AccessModes(
    set_is_init: WriteSignal<bool>,
    set_is_verify: WriteSignal<bool>
) -> impl IntoView {
    let (input_password, set_input_password) = create_signal(String::new());
    let (is_incorrect, set_is_incorrect) = create_signal(false);
    let (use_password, set_use_password) = create_signal(false);
    let verify_action = Action::new(move |_| {
        async move {
            let result = verify_password_api(input_password.get()).await;
            match result {
                Ok(true) => {
                    set_is_incorrect(false);
                    set_is_verify(true);
                    set_is_init(true);
                    show_info_toast("Admin Mode", "Welcome Admin user.");
                }
                _ => {
                    show_error_toast("Failed", "Incorrect Password.");
                    set_is_incorrect(true);
                }
            }
        }
    });
    view! {
        <Topbar/>
        <div class="selectMode" >  <b><h1 style="font-size: 1.5rem;">"Edit Page"</h1></b>
        <div style="display: flex; flex-direction: column; margin-top: 15px; gap: 1rem">
        <b style="font-size: 18px; text-align:center">Select Access Mode</b>
        <button 
        type="button"
        style="width: 20rem; height: 2.5rem; margin-top: 1rem; color:green;   border-width: 1px;  border-color: green;"
        on:click=move |_| {
           show_info_toast("Viewer Mode", "Welcome Viewer user.");
            set_is_init(true);     
        }
        >Viewer Mode "(can't update)"</button>
        <button 
        type="button"
        style="width: 20rem; height: 2.5rem;    border-width: 1px;  border-color: blue;"
        on:click=move |_| {
             set_use_password(true);
        }
        >Admin Mode</button>
        </div>
        { move || if use_password.get() {
            view! {
                <div style="width: 20rem; margin-top: 30px;">
                <InputField input_type="password" 
                id="input_password" 
                label="Admin Password" 
                set_value=set_input_password  
                get_value=input_password 
                require=true />
             <p style="color:red;">    {move || if is_incorrect.get() { "Incorrect Password" } else { "" }}</p>
                 <div class="formButton">
                <button
                    type="button"
                    class="updateButton"
                    on:click=  move |_| {
                        verify_action.dispatch(());
                    }>
                    Verify
                </button>   
            </div>  
                </div>        
            } }
        else{
        view! {
        <div></div>
        }} }                       
        </div> 
    }
}
