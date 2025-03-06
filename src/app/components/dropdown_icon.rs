use leptos::*;
use leptos_icons::Icon;
use crate::app::utils::ICON_MAP;
#[component]
pub fn IconDropdown() -> impl IntoView {
    // Signal for the currently selected icon name (if any)
    let (selected_icon, set_selected_icon) = create_signal(None::<&'static str>);
    // Signal for whether the dropdown is open
    let (is_open, set_is_open) = create_signal(false);

    view! { 
        <div class="icon-dropdown" style="position: relative; display: inline-block;">
            <button 
            type="button"
                on:click=move |_| set_is_open.update(|open| *open = !*open)
                style="display: flex; align-items: center; gap: 8px;"
            >
                {
                    move || {
                        // If an icon is selected, show its icon and name; otherwise, show a placeholder.
                        if let Some(name) = selected_icon.get() {
                            if let Some(&icon) = ICON_MAP.get(name) {
                                view! { 
                                    <div class="aLinkRow"> 
                                    <Icon icon={icon} />
                                    <span style="margin-left: 8px" >{name}</span>
                                    </div>
                                }
                            } else {
                                view! {   <div>  "Select an icon" </div> }
                            }
                        } else {
                            view! { <div>  "Select an icon" </div> }
                        }
                    }
                }
            </button>
            { move || if is_open.get() {
                view! { 
                 
                    <ul 
                        style="
                            position: absolute;
                            top: 100%;
                            left: 0;
                            background: white;
                            list-style: none;
                            margin: 0;
                            padding: 0;
                            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
                        "
                    >
                        { ICON_MAP.into_iter().map(|(&name, &icon)| {
                            view! { 
                                <li 
                                    on:click=move |_| {
                                        set_selected_icon.set(Some(name));
                                        set_is_open.set(false);
                                    }
                                    style="
                                        padding: 8px;
                                        cursor: pointer;
                                        display: flex;
                                        align-items: center;
                                        gap: 8px;
                                    "
                                >
                                    <Icon icon={icon} />
                                    <span>{name}</span>
                                </li>
                            }
                        }).collect::<Vec<_>>() }
                    </ul>
                  
                }
            } else {
                view! {  <ul></ul> }
            } }
        </div>
    }
}
