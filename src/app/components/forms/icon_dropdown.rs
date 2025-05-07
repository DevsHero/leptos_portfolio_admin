use leptos::*;
use leptos_icons::Icon;
use icondata as i;
use crate::app::utils::utils::ICON_MAP;
#[component]
pub fn IconDropdown(
    label: impl Into<String>,
    set_value: WriteSignal<String>,
    get_value: ReadSignal<String>,

    require: bool,
    #[prop(optional)] validation: Option<ReadSignal<bool>>
) -> impl IntoView {
    let label = label.into();
    let label_for_error = label.clone();
    let (selected_icon, set_selected_icon) = create_signal(String::new());
    let (is_open, set_is_open) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);

    create_effect(move |_| {
        let current_value = get_value.get();
        if !current_value.is_empty() {
            if selected_icon.get() != current_value {
                set_selected_icon.set(current_value);
            }
        }
    });

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
            if trigger.get() {
                validate();
            }
        });
    }
    let renderLabel = if require { format!("{}*", label) } else { format!("{}", label) };
    view! { 
        <>
        <div  style="display: flex;flex-direction: row;">
 
            <label style="margin-right: 52px; margin-bottom: 15px;" >{renderLabel}</label>    
                <div style="position: relative;">
                    <button     
                        type="button"
                        on:click=move |_| set_is_open.update(|open| *open = !*open)
                          style="display: flex; align-items: center; gap: 8px;"
                    >
                        {
                            move || {
                                let icon_name = selected_icon.get();
                                if !get_value.get().is_empty()   && !icon_name.is_empty() {
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
                                                set_value.set(name_string.clone());
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
