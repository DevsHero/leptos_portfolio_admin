use leptos::{ either::Either, prelude::* };

#[component]
pub fn TextAreaField(
    set_value: WriteSignal<String>,
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
    let (error, set_error) = signal(None::<String>);
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
        Effect::new(move |_| {
            if trigger.get() {
                validate();
            }
        });
    }
    let renderLabel = if require { format!("{}*", label) } else { format!("{}", label) };
    (
        view! {
        <div class="formGroup">
            <label for={id.clone()}>{renderLabel}</label>
            <textarea
                id={id.clone()}
                prop:value=move || get_value.get()
                on:input=move |ev| {
                    let value = event_target_value(&ev);
                    set_value(value.clone());
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
                     Either::Left(    view! { <p class="errorInput">{msg}</p> })
                    } else {
                        Either::Right(())
                    }
                }
            }
        </div>
    }
    ).into_any()
}
