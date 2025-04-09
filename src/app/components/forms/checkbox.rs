use leptos::*;

#[component]
pub fn CheckBox(
    set_value: WriteSignal<bool>,
    get_value: ReadSignal<bool>,
    id: impl Into<String>,
    label: impl Into<String>
) -> impl IntoView {
    let id = id.into();
    let label = label.into();

    view! {
        <div class="formGroup">
            <div class="checkboxRow">
                <label style="width:100%;" id={id.clone()}>{label}</label>
                <input
                    style="margin-left: 30px; border-width: 1px;"
                    type="checkbox"
                    id={id}
                    prop:checked=move || get_value.get()
                    on:change=move |ev| {
                        let checked = event_target_checked(&ev);
                        set_value(checked);
                    }
                />
            </div>
        </div>
    }
}
