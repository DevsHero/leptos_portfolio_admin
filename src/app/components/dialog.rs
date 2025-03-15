use leptos::*;
#[component]
pub fn Dialog(title: String, detail: String, children: Children) -> impl IntoView {
    let (show_dialog, set_show_dialog) = create_signal(false);

    let toggle_dialog = move |_|
        set_show_dialog.update(|open| {
            *open = !*open;
        });

    view! {
        <div class="dialog-container">
            <button 
                type="button" 
                class="dialog-trigger" 
                on:click=toggle_dialog
            >
                {children()}
            </button>
            
            {move || show_dialog.get().then(|| view! {
                <div class="dialog-overlay" on:click=toggle_dialog>
            
                    <div 
                        class="dialog-content"
                        on:click=move |e| e.stop_propagation()
                    >
                        <div class="dialog-header">
                            <div class="header-with-icon">
                                <div class="icon-container">
                              
                                </div>
                             
                                <h3  class="dialog-title">  {title.clone()} </h3>
                            </div>
                            <button 
                                class="dialog-close" 
                                on:click=toggle_dialog
                            >
                             "×"
                            </button>
                        </div>
                        <div class="dialog-body">
                            <p>{detail.clone()}</p>
                        </div>
                    </div>
                </div>
            })}
        </div>
    }
}
