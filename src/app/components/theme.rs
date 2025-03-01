use crate::app::icons;
use crate::app::utils::{ getItem, setItem };
use leptos::*;
use leptos::{ component, view, IntoView };
use wasm_bindgen::JsValue;
use web_sys::window;
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn update_local_storage(new_value: &str) {
    setItem("mode", new_value);
}
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn get_mode() -> JsValue {
    getItem("mode")
}
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
    create_effect(move |_| {
        let prefered_mode = get_mode();
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
            view! { <icons::Moon /> }
        } else {
            view! { <icons::Sun /> }
        }
    };
    view! {
        <button
            on:click=move |_| {
                set_dark_mode(!dark_mode());
                let new_value = if dark_mode() { "dark" } else { "light" };
                update_local_storage(new_value);
                darkmode(dark_mode());
            }
            class="topbarButton"
        >
            {icon}
        </button>
    }
}
