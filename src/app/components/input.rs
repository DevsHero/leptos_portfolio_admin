use leptos::*;
use leptos::{ component, view, IntoView };
#[component]
pub fn InputField(
    setField: ReadSignal<String>,
    getValue: ReadSignal<String>,
    id: &String,
    label: String,
    require: bool,
    is_row: bool
) -> impl IntoView {
    let class = Memo::new(move |_| {
        if !is_row { "formGroup" } else { "formRow" }
    });
    let (error, set_error) = create_signal(None::<String>);
    view! {
        <div class=class >
        <label for=id>{label}</label>
        <input
            type="text"
            id=id
           prop:value=move || getValue.get()
            on:input=move |ev| {
                let value = event_target_value(&ev);
                setField(value.clone());
                // Optionally, perform live validation:
                if value.trim().is_empty() {
                    set_error(Some({label} " is required.".to_string()));
                } else {
                    set_error(None);
                }
            }
            
        />
       if require {move || {
            if let Some(msg) = error.get() {
                view! { <p class="errorInput">{msg}</p> }
            } else {
                view! { <p class="errorInput">{}</p> }
            }
        }}
       
        </div>
        }
}
