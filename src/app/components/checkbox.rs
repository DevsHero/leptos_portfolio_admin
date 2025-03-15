use leptos::*;
use leptos::{ component, view, IntoView };

#[component]
pub fn CheckBox(
    set_field: WriteSignal<bool>,
    get_value: ReadSignal<bool>,
    id: impl Into<String>,
    label: impl Into<String>
) -> impl IntoView {
    let id = id.into();
    let label = label.into();

    view! {
     
        <div class="formGroup">
        <div class="checkboxRow">
            <label for={id.clone()}>{label}</label>
            <input
            style="margin-left : 20px;"
                type="checkbox"
                id={id.clone()}
                prop:value=move || get_value.get()
                on:input=move |_ev| {
                    let checkValue = get_value.clone();
                    set_field(!checkValue.get()); 
                }
            />
           
        </div>
        </div>
    }
}
