use leptos::*;
use leptos::{ component, view, IntoView };

use web_sys::window;
use leptos_icons::Icon;
use icondata as i;
fn darkmode(enable: bool) {
    let window = window().expect("Failed to get window");
    let document = window.document().expect("Failed to get document");
    let body = document.body().expect("Failed to get body");
    if enable {
        let _ = body.class_list().add_1("dark");
    } else {
        let _ = body.class_list().remove_1("dark");
    }
}
#[component]
pub fn ThemeButton() -> impl IntoView {
    let (dark_mode, set_dark_mode) = create_signal(false);
    let icon = move || {
        if dark_mode() {
            view! { <Icon icon={i::MdiWeatherNight} /> }
        } else {
            view! { <Icon icon={i::FiSun} /> }
        }
    };
    view! {
        <button
            on:click=move |_| {
                set_dark_mode(!dark_mode());
                let _new_value = if dark_mode() { "dark" } else { "light" };
                // update_local_storage(new_value);
                darkmode(dark_mode());
            }
            class="topbarButton"
        >
            {icon}
        </button>
    }
}
