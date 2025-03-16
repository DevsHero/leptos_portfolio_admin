use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::js_sys;

#[wasm_bindgen]
extern "C" {
    type LeptosEditors;
    #[wasm_bindgen(js_namespace = window)]
    fn leptosEditors() -> Option<LeptosEditors>;
    #[wasm_bindgen(method, js_name = "init")]
    fn init(this: &LeptosEditors, selector: &str, initial_content: &str, callback: &JsValue);
    #[wasm_bindgen(method, js_name = "setContent")]
    fn set_content(this: &LeptosEditors, editor_id: &str, content: &str);
    #[wasm_bindgen(method, js_name = "destroy")]
    fn destroy(this: &LeptosEditors, editor_id: &str);
}

#[component]
pub fn TextEditor(
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(into)] value: Signal<String>,
    #[prop(into)] set_value: WriteSignal<String>
) -> impl IntoView {
    let editor_id = id.unwrap_or_else(|| {
        let random = js_sys::Math::random().to_string();
        let random = random.split('.').last().unwrap_or("0");
        format!("editor-{}", random)
    });

    let editor_selector = format!("#{}", &editor_id);
    let initial_content = value.get();
    let editor_selector_clone = editor_selector.clone();
    let editor_id_for_effect = editor_id.clone();

    create_effect(move |_| {
        let content = value.get();
        if let Some(editors) = leptosEditors() {
            editors.set_content(&editor_id_for_effect, &content);
        }
    });

    let on_change_js = {
        let closure = Closure::wrap(
            Box::new(move |content: String| {
                set_value.set(content);
            }) as Box<dyn Fn(String)>
        );
        let js_value = closure.as_ref().clone();
        closure.forget(); // Prevent Rust from dropping the Closure
        js_value
    };
    let editor_id_for_cleanup = editor_id.clone();
    create_effect(move |_| {
        let window = web_sys::window().expect("no global window exists");

        // Clone the JsValue for use in callback
        let callback_js = on_change_js.clone();
        let editor_selector_clone = editor_selector.clone();
        let initial_content_clone = initial_content.clone();

        let callback = Closure::wrap(
            Box::new(move || {
                if let Some(editors) = leptosEditors() {
                    editors.init(
                        &editor_selector_clone,
                        &initial_content_clone,
                        &callback_js // Use cloned JsValue
                    );
                }
            }) as Box<dyn FnMut()>
        );

        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            500
        );
        callback.forget();

        {
            let value = editor_id_for_cleanup.clone();
            move || {
                if let Some(editors) = leptosEditors() {
                    editors.destroy(&value);
                }
            }
        }
    });

    let is_disabled = disabled.unwrap_or(false);
    view! {
        <div class=class id=editor_id>
            <textarea 
                style="visibility: hidden; display: none;" 
                disabled=is_disabled
            >
                {move || value.get()}
            </textarea>
        </div>
    }
}
