use leptos::*;
use leptos::{ component, view, IntoView };

#[component]
pub fn TextAreaField(
    set_field: WriteSignal<String>,
    get_value: ReadSignal<String>,
    id: impl Into<String>,
    label: impl Into<String>,
    require: bool,
    #[prop(optional)] validation: Option<ReadSignal<bool>>
) -> impl IntoView {
    let id = id.into();
    let label = label.into();
    let error_label = label.clone();
    let label_for_input = label.clone();
    let (error, set_error) = create_signal(None::<String>);
    // Create a function to validate the input
    let validate = move || {
        let value = get_value.get();
        if require && value.trim().is_empty() {
            set_error(Some(format!("{} is required.", error_label.clone())));
            false
        } else {
            set_error(None);
            true
        }
    };

    // If a validation trigger is provided, create an effect to watch it
    if let Some(trigger) = validation {
        create_effect(move |_| {
            // When the trigger changes to true, perform validation
            if trigger.get() {
                validate();
            }
        });
    }
    view! {
        <div class="formGroup">
            <label for={id.clone()}>{label}</label>
            <textarea
                type="text"
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
                        view! { <p class="errorInput">{msg}</p> }
                    } else {
                        view! { <p ></p> }
                    }
                }
            }
        </div>
    }
}
