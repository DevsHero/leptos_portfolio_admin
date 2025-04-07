use leptos::*;
use leptos_toaster::{ Theme, Toast, ToastId, ToastOptions, ToastVariant, ToasterPosition, Toasts };

use crate::app::{ components::{ InputField, Topbar }, server::api::verify_password_api };

#[component]
pub fn AccessModes(
    set_is_init: WriteSignal<bool>,
    set_is_verify: WriteSignal<bool>,
    get_verify: ReadSignal<bool>
) -> impl IntoView {
    let (input_password, set_input_password) = create_signal(String::new());
    let (is_incorrect, set_is_incorrect) = create_signal(false);
    let (use_password, set_use_password) = create_signal(false);
    let create_toast = move |title: View, detail: View, varaint: ToastVariant| {
        let toast_id = ToastId::new();
        let toast_context = expect_context::<Toasts>();
        toast_context.toast(
            view! {
                <Toast
                    toast_id
                    variant=varaint
                    theme=Theme::Dark
                    invert=false
                    rich_colors=false
                    title=view! { {title} }.into_view()
                    description=Some(view! {  {detail}}.into_view())
                />
            },
            Some(toast_id),
            Some(ToastOptions {
                dismissible: true,
                duration: Some(std::time::Duration::from_secs(4)),
                position: Some(ToasterPosition::BottomLeft),
            })
        );
    };
    let verify_action = Action::new(move |_| {
        async move {
            let result = verify_password_api(input_password.get()).await;
            match result {
                Ok(true) => {
                    set_is_incorrect(false);
                    set_is_verify(true);
                    set_is_init(true);
                    create_toast(
                        (
                            {
                                view! { <p class="toastInfo">"Admin Mode" </p> }
                            }
                        ).into_view(),
                        "Welcome Admin user.".into_view(),
                        ToastVariant::Info
                    );
                }
                _ => {
                    create_toast(
                        (
                            {
                                view! { <p class="toastFail">"Failed" </p> }
                            }
                        ).into_view(),
                        "Incorrect Password.".into_view(),
                        ToastVariant::Error
                    );

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
            create_toast({view! {<p class="toastInfo">"Viewer Mode" </p>}}.into_view(), "Welcome Viewer user.".into_view(), ToastVariant::Info);
          
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
        {if use_password.get() {
            view! {
                <div style="width: 20rem; margin-top: 30px;">
                <InputField input_type="password" id="input_password" label="Admin Password" set_value=set_input_password  get_value=input_password require=true />
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
