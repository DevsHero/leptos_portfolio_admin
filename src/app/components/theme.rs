use leptos::{ either::Either, prelude::* };

use web_sys::window;
use leptos_icons::Icon;
use icondata as i;
use crate::app::utils::{ getLocalStorage, setLocalStorage };
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
    let (dark_mode, set_dark_mode) = signal(false);
    Effect::new(move |_| {
        let prefered_mode = getLocalStorage("mode");

        let mode = match prefered_mode.as_string() {
            Some(mode) => mode,
            None => "dark".to_string(),
        };

        let value = mode.as_str() == "dark";

        set_dark_mode(value);
        darkmode(value);
    });
    let icon = move || {
        if dark_mode() {
            Either::Left(view! { <Icon icon={i::MdiWeatherNight} /> })
        } else {
            Either::Right(view! { <Icon icon={i::FiSun} /> })
        }
    };
    (
        view! {
        <button
        type="button" 
        on:click=move |_| {
            set_dark_mode(!dark_mode());
            let new_value = if dark_mode() { "dark" } else { "light" };
            setLocalStorage("mode" , new_value);
            darkmode(dark_mode());
        }
            class="topbarButton"
        >
            {icon}
        </button>
    }
    ).into_any()
}
