use leptos::*;

#[component]
pub fn InputField(
    set_value: WriteSignal<String>,
    get_value: ReadSignal<String>,
    id: impl Into<String>,
    label: impl Into<String>,
    input_type: impl Into<String>,
    require: bool,
    #[prop(optional)] validation: Option<ReadSignal<bool>>
) -> impl IntoView {
    let id = id.into();
    let label = label.into();
    let input_type = input_type.into();
    let error_label = label.clone();
    let label_for_input = label.clone();
    let (error, set_error) = create_signal(None::<String>);

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

    if let Some(trigger) = validation {
        create_effect(move |_| {
            if trigger.get() {
                validate();
            }
        });
    }
    let renderLabel = if require { format!("{}*", label) } else { format!("{}", label) };
    view! {
        <div class="formGroup">
            <label id={id.clone()}>{renderLabel}</label>
            <input
                type={input_type.clone()}
                id={id.clone()}
                prop:value=move || get_value.get()
                on:input=move |ev| {
                    let value = event_target_value(&ev);
                    set_value(value.clone());
                    if require && value.trim().is_empty() {
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
