use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::js_sys;

#[wasm_bindgen]
extern "C" {
    // Initialize TinyMCE editor
    #[wasm_bindgen(js_namespace = window, js_name = init_tiny_mce)]
    fn init_tiny_mce(selector: &str, initial_content: &str, callback: &JsValue);

    // Set editor content
    #[wasm_bindgen(js_namespace = window, js_name = set_tiny_mce_content)]
    fn set_tiny_mce_content(editor_id: &str, content: &str);

    // Destroy editor instance
    #[wasm_bindgen(js_namespace = window, js_name = destroy_tiny_mce)]
    fn destroy_tiny_mce(editor_id: &str);
}

#[component]
pub fn TextEditor2(
    #[prop(optional)] class: Option<impl Into<String>>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(into)] get_value: Signal<String>,
    #[prop(into)] set_value: WriteSignal<String>,
    #[prop(optional)] validation: Option<ReadSignal<bool>>,
    id: impl Into<String>,
    require: bool,
    label: impl Into<String>
) -> impl IntoView {
    let editor_id = id.into();
    let label = label.into();
    let class = class.unwrap();
    let error_label = label.clone();
    let error2_label = label.clone();
    let (error, set_error) = create_signal(None::<String>);
    // Create a function to validate the input
    let validate = move || {
        let get_value = get_value.get();
        if require && get_value.trim().is_empty() {
            set_error(Some(format!("{} is required.", error_label.clone())));
            false
        } else {
            set_error(None);
            true
        }
    };
    if let Some(trigger) = validation {
        create_effect(move |_| {
            // When the trigger changes to true, perform validation
            if trigger.get() {
                validate();
            }
        });
    }
    let editor_selector = format!("#{}", &editor_id);
    let initial_content = get_value.get();
    let editor_id_clone = editor_id.clone();
    let editor_id_for_cleanup = editor_id.clone();
    let on_change = {
        let closure = Closure::wrap(
            Box::new(move |content: String| {
                if require && content.trim().is_empty() {
                    set_error(Some(format!("{} is required.", error2_label)));
                } else {
                    set_error(None);
                }
                set_value.set(content);
            }) as Box<dyn Fn(String)>
        );
        let js_value = closure.as_ref().clone();

        closure.forget(); // Prevent Rust from dropping the closure
        js_value
    };

    create_effect(move |_| {
        let window = web_sys::window().expect("no global window exists");
        let document = window.document().expect("no document exists");

        // Check if our style is already added
        if document.query_selector(".tinymce-no-promotion-style").ok().flatten().is_none() {
            if let Ok(Some(head)) = document.query_selector("head") {
                if let Ok(style_element) = document.create_element("style") {
                    let _ = style_element.set_attribute("class", "tinymce-no-promotion-style");
                    let _ = style_element.set_attribute("type", "text/css");
                    style_element.set_text_content(
                        Some(".tox-promotion { display: none !important; }")
                    );
                    let _ = head.append_child(&style_element);
                }
            }
        }

        // This is important for editor initialization
        let window_clone = window.clone();
        let editor_selector = editor_selector.clone();
        let initial_content = initial_content.clone();
        let on_change = on_change.clone();

        // Use a slightly longer timeout to ensure DOM is fully ready
        let callback = Closure::wrap(
            Box::new(move || {
                init_tiny_mce(&editor_selector, &initial_content, &on_change);
            }) as Box<dyn FnMut()>
        );

        let _ = window_clone.set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            250 // Slightly longer delay for better initialization
        );

        callback.forget();
    });
    create_effect(move |_| {
        let content = get_value.get();
        set_tiny_mce_content(&editor_id, &content);
    });
    on_cleanup(move || {
        destroy_tiny_mce(&editor_id_clone);
    });
    // hidden upgrade button from tinymce
    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("no document exists");
    if let Ok(Some(head)) = document.query_selector("head") {
        if let Ok(style_element) = document.create_element("style") {
            if let Err(_) = style_element.set_attribute("type", "text/css") {
            }
            style_element.set_text_content(Some(".tox-promotion { display: none !important; }"));
            let _ = head.append_child(&style_element);
        }
    }

    view! {
        <div class="formGroup">
        <label  >{label}</label>
        <div class=class.into() id=editor_id_for_cleanup>
            <textarea 
                style="visibility: hidden; display: none;" 
                disabled=disabled.unwrap_or(false)
            >
                {move || get_value.get()}
            </textarea>
        </div>
        {
            move || {
                if let Some(msg) = error.get() {
                    view! { <p class="errorInput">{msg}</p> }
                } else {
                    view! { <p ></p> }
                }
            }
        }
        </div>
    }
}
