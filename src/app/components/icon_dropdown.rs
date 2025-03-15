use leptos::*;
use leptos_icons::Icon;
use icondata as i;
use crate::app::utils::ICON_MAP;
#[component]
pub fn IconDropdown(
    label: impl Into<String>,
    set_field: WriteSignal<String>,
    get_value: ReadSignal<String>,

    require: bool,
    #[prop(optional)] validation: Option<ReadSignal<bool>>
) -> impl IntoView {
    let label_text = label.into();
    let label_for_error = label_text.clone();
    // Store the selected icon as a String instead of &'static str
    let (selected_icon, set_selected_icon) = create_signal(String::new());
    let (is_open, set_is_open) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);

    // Initialize selected icon from the current value when component mounts
    create_effect(move |_| {
        let current_value = get_value.get();
        if !current_value.is_empty() {
            // Only update if we have a value and it's different
            if selected_icon.get() != current_value {
                set_selected_icon.set(current_value);
            }
        }
    });

    // Validation function
    let validate = move || {
        let value = get_value.get();
        if require && value.is_empty() {
            set_error.set(Some(format!("{} is required.", label_for_error.clone())));
            false
        } else {
            set_error.set(None);
            true
        }
    };

    if let Some(trigger) = validation {
        create_effect(move |_| {
            // When the trigger changes, perform validation
            if trigger.get() {
                validate();
            }
        });
    }

    view! { 
        <>
        <div  style="display: flex;flex-direction: row;">
 
            <label style="margin-right: 52px; margin-bottom: 15px;" >{label_text}</label>    
                <div style="position: relative;">
                    <button     
                        type="button"
                        on:click=move |_| set_is_open.update(|open| *open = !*open)
                          style="display: flex; align-items: center; gap: 8px;"
                    >
                        {
                            move || {
                                let icon_name = selected_icon.get();
                                if !icon_name.is_empty() {
                                    // Look up the icon using the string name
                                    if let Some(&icon) = ICON_MAP.get(icon_name.as_str()) {
                                        view! { 
                                            <div class="aLinkRow"> 
                                            <Icon icon={icon} />
                                            <span style="margin-left: 8px" >{icon_name}</span>
                                            </div>
                                        }
                                    } else {
                                        view! { <div class="dropdown"><Icon icon={i::AiSearchOutlined} /> <span class="ml-2">"Select icon"</span></div> }
                                    }
                                } else {
                                    view! { <div class="dropdown"><Icon icon={i::AiSearchOutlined} /> <span class="ml-2">"Select icon"</span></div> }
                                }
                            }
                        }
                    </button>
                    { move || if is_open.get() {
                        view! { 
                            <ul 
                                class="absolute top-full left-0 z-10 w-full bg-white shadow-lg rounded-md overflow-y-auto max-h-60 border border-gray-200"
                                style="color: var(--background); position:absolute; padding:0.5rem; border-radius = 5; min-width: 200px; max-height: 200px; overflow-y: scroll;  background-color: var(--lavender);"
                            >
                                { ICON_MAP.into_iter().map(|(&name, &icon)| {
                                    let name_string = name.to_string();
                                    view! { 
                                        <li 
                                            on:click=move |_| {
                                                set_selected_icon.set(name_string.clone());
                                                set_is_open.set(false);
                                                set_field.set(name_string.clone());
                                                // Clear error if a value is selected
                                                set_error.set(None);
                                            }
                                            class="px-3 py-2 hover:bg-gray-100 cursor-pointer flex items-center gap-2"
                                        >
                                            <Icon style="margin-right:5px;" icon={icon} />
                                            <span >{name}</span>
                                        </li>
                                    }
                                }).collect::<Vec<_>>() }
                            </ul>
                        }
                    } else {
                        view! { <ul class="hidden"></ul> }
                    } }
                </div>
        
        
        </div>
        {
            move || {
                if let Some(msg) = error.get() {
                    view! { <p class="errorInput">{msg}</p> }
                } else {
                    view! { <p></p> }
                }
            }
        }
        </>
    }
}
