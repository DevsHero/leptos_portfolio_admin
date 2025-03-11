use leptos::*;
use leptos::{ component, view, IntoView };

#[component]
pub fn InputField(
    set_field: WriteSignal<String>,
    get_value: ReadSignal<String>,
    id: impl Into<String>,
    label: impl Into<String>,
    require: bool,
    input_type: impl Into<String>
) -> impl IntoView {
    let id = id.into();
    let label = label.into();
    let input_type = input_type.into();
    let label_for_input = label.clone();
    let (error, set_error) = create_signal(None::<String>);
    view! {
        <div class="formGroup">
            <label for={id.clone()}>{label}</label>
            <input
                type={input_type.clone()}
                id={id.clone()}
                prop:value=move || get_value.get()
                on:input=move |ev| {
                    let value = event_target_value(&ev);
                    set_field(value.clone());
                    // Optionally perform live validation:
                    if value.trim().is_empty() {
                        set_error(Some(format!("{} is required.", label_for_input.clone())));
                    } else {
                        set_error(None);
                    }
                }
            />
            {
                move || {
                    if let Some(msg) = error.get() {
                        if require {  
                            view! { <p class="errorInput">{msg}</p> }
                        } else {
                            view! { <p class="errorInput"></p> }
                        }
                    } else {
                        view! { <p class="errorInput"></p> }
                    }
                }
            }
        </div>
    }
}
