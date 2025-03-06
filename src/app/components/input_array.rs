use leptos::*;
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
    // Clone label for use inside the on:input closure.

    view! {
                    <div class="formGroup">
                    <div class="experienceRow">
                        <label for={id.clone()}>{label}</label>
                  
                    
                        <button
                        type="button"
                        class="absolute left-0 top-1/2 transform -translate-y-1/2 bg-gray-800 text-white px-4  opacity-75 hover:opacity-100"
    
                        on:click=move |ev| { 
                            set_fields.update(|field| field.push("".to_string()));
                        }
                    >
                    <Icon icon={i::CgAdd} />
                    </button>
                 
                    </div>
                        {move ||
                            get_values
                                .get()
                                .into_iter()
                                .enumerate()
                                .map(|(index, get_value)| {
                         let (error, set_error) = create_signal(None::<String>);
                         
                                    view! {  
                                        <div class="inputArrayRow">
                                        <button
                        type="button"
                       class="absolute left-0 top-1/2 transform -translate-y-1/2 bg-gray-800 text-white px-4  opacity-75 hover:opacity-100"
                       disabled={get_values.get().len() == 1}
                        on:click=move |_ev| { 
                            set_fields.update(|values| {
                                if index < values.len() {
                                    values.remove(index);
                                }
                            });
                        }
                    >
                    <Icon icon={i::CgRemove} />
                    </button>
                           <input
                           class="w-full"
                            type="text"
                            id={id.clone()}
                            prop:value=move || get_value.clone()
                      
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_fields.update(|fields| {
                                    if let Some(field) = fields.get_mut(index) {
                                        *field = value.clone();
                                    }
                                });
                            
                                if value.trim().is_empty() {
                                    set_error(Some(format!("is required.")));
                                } else {
                                    set_error(None);
                                }
                            }
                        /> 
                        
                    </div>
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
                        }}
                    })
                    .collect::<Vec<_>>()
            }
                    </div>
                }
}
