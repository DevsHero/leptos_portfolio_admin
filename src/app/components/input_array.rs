use leptos::prelude::*;
use leptos::{ component, view, IntoView };
use leptos_icons::Icon;
use icondata as i;

#[component]
pub fn InputArrayField(
    set_fields: WriteSignal<Vec<String>>,
    get_values: ReadSignal<Vec<String>>,
    id: impl Into<String>,
    label: impl Into<String>,
    require: bool
) -> impl IntoView {
    let id = id.into();
    let label = label.into();
    (
        view! {
        <div class="formGroup">
            <div class="experienceRow">
                <label for={id.clone()}>{label}</label>
                <button
                    type="button"
                    class="btn-add"
                    on:click=move |_| {
                        set_fields.update(|fields| fields.push("".into()));
                    }
                >
                    <Icon icon={i::CgAdd} />
                </button>
            </div>
            
            <For
            each=move || get_values.get().into_iter().enumerate()
            key=|(idx, _)| *idx
            children=move |(index, _)| {
                let input_id = format!("{}-{}", id, index);
                view! {
                    <div class="inputArrayRow" id={index}>
                        <button
                            type="button"
                            class="btn-remove"
                            disabled={get_values.get().len() == 1}
                            on:click=move |_| {
                                set_fields.update(|fields| { fields.remove(index); });
                            }
                        >
                            <Icon icon={i::CgRemove} />
                        </button>
                        <input
                            type="text"
                            id={input_id}
                            class="w-full"
                            prop:value=move || get_values.get().get(index).cloned().unwrap_or_default()
                            on:input=move |ev| {
                                let val = event_target_value(&ev);
                                set_fields.update(|fields| {
                                    if let Some(field) = fields.get_mut(index) {
                                        *field = val;
                                    }
                                });
                            }
                        />
                        {move || require.then(|| {
                            let is_empty = get_values.get()
                                .get(index)
                                .map(|v| v.trim().is_empty())
                                .unwrap_or(true);
                            is_empty.then(|| view! { <p class="errorInput">"Required"</p> })
                        })}
                    </div>
                }
            }
        />
        </div>
    }
    ).into_any()
}
