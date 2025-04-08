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
    let (is_restricted, set_is_restricted) = create_signal(false);
    let (use_password, set_use_password) = create_signal(false);

    let verify_action = Action::new(move |_| {
        async move {
            let result = verify_password_api(input_password.get()).await;
            match result {
                Ok(verification) => {
                    if verification.restrict {
                        // Handle rate limit restriction
                        set_is_restricted(true);
                        show_error_toast(
                            "Rate Limited",
                            "Too many attempts. Please try again later."
                        );
                    } else if verification.verify {
                        // Password verified successfully
                        set_is_incorrect(false);
                        set_is_verify(true);
                        set_is_init(true);
                        show_info_toast("Admin Mode", "Welcome Admin user.");
                    } else {
                        // Password incorrect
                        show_error_toast("Failed", "Incorrect Password.");
                        set_is_incorrect(true);
                    }
                }
                _ => {
                    show_error_toast("Failed", "Verification failed. Please try again.");
                    set_is_incorrect(true);
                }
            }
        }
    });

    view! {
        <Topbar/>
        <div class="selectMode" >
            <b><h1 style="font-size: 1.5rem;">"Edit Page"</h1></b>
            <div style="display: flex; flex-direction: column; margin-top: 15px; gap: 1rem">
                <b style="font-size: 18px; text-align:center">Select Access Mode</b>
                <button 
                    type="button"
                    style="width: 20rem; height: 2.5rem; margin-top: 1rem; color:green; border-width: 1px; border-color: green;"
                    on:click=move |_| {
                        show_info_toast("Viewer Mode", "Welcome Viewer user.");
                        set_is_init(true);     
                    }
                >
                    Viewer Mode "(can't update)"
                </button>
                <button 
                    type="button"
                    style="width: 20rem; height: 2.5rem; border-width: 1px; border-color: blue;"
                    on:click=move |_| {
                        set_use_password(true);
                        set_is_restricted(false); 
                    }
                >
                    Admin Mode
                </button>
            </div>
            
            {move || {
                if use_password.get() {
                    if is_restricted.get() {
                        // Show rate limit message when restricted
                        view! {
                            <div style="width: 20rem; margin-top: 30px; text-align: center;">
                                <div style="padding: 15px; border: 1px solid #ffcccc; background-color: #fff8f8; border-radius: 5px;">
                                    <p style="color: #cc0000; font-weight: bold; margin-bottom: 10px;">Access Temporarily Restricted</p>
                                    <p style="color: #666666;">
                                        "Too many failed login attempts detected. For security reasons, admin access has been temporarily disabled."
                                    </p>
                                    <p style="color: #666666; margin-top: 10px;">
                                        "Please try again later after 5 minutes."
                                    </p>
                                </div>
                            </div>
                        }
                    } else {
                        // Show password input when not restricted
                        view! {
                            <div style="width: 20rem; margin-top: 30px;">
                                <InputField 
                                    input_type="password" 
                                    id="input_password" 
                                    label="Admin Password" 
                                    set_value=set_input_password  
                                    get_value=input_password 
                                    require=true 
                                />
                                <p style="color:red;">
                                    {move || if is_incorrect.get() { "Incorrect Password" } else { "" }}
                                </p>
                                <div class="formButton">
                                    <button
                                        type="button"
                                        class="updateButton"
                                        on:click=move |_| {
                                            verify_action.dispatch(());
                                        }
                                    >
                                        Verify
                                    </button>   
                                </div>  
                            </div>        
                        }
                    }
                } else {
                    view! { <div></div> }
                }
            }}                      
        </div> 
    }
}
