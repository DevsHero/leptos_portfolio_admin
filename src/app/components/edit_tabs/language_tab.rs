use crate::app::components::{
    show_error_toast,
    show_success_toast,
    InputField,
    LanguageChips,
    RenderTab,
};

use crate::app::constants::constant::LANGUAGE_LEVELS;
use crate::app::models::Language;
use leptos::*;

#[component]
pub fn EditLanguageTab(
    languages: ReadSignal<Vec<Language>>,
    set_languages: WriteSignal<Vec<Language>>,
    set_is_update_language: WriteSignal<bool>,
    select_tab: ReadSignal<i32>
) -> impl IntoView {
    let (language_name, set_language_name) = create_signal(String::new());
    let (language_level, set_language_level) = create_signal(String::from("1"));
    let (validate_language, set_validate_language) = create_signal(false);
    let add_language = move |_| {
        set_validate_language.update(|v| {
            *v = !*v;
        });
        let form_valid = !language_name.get().trim().is_empty();
        if form_valid {
            let new_language = Language {
                name: language_name.get(),
                level: language_level.get(),
            };
            set_languages.update(|languages| languages.push(new_language));
            set_validate_language.set(false);
            set_language_name.set(String::new());
            set_language_level.set(String::from("Intermediate"));
            set_is_update_language(true);
            show_success_toast("Add Language Success", "Language Added.");
        } else {
            show_error_toast("Add Language Failed", "Missing required field.");
        }
    };
    let delete_language = move |index: usize| {
        set_languages.update(|languages| {
            languages.remove(index);
        });
        set_is_update_language(true)
    };

    let edit_language = move |index: usize| {
        let list = languages.get();
        if
            let Some(language) = list
                .iter()
                .enumerate()
                .find(|(i, _)| *i == index)
        {
            let language = language.1.clone();
            set_language_name.set(language.name);
            set_language_level.set(language.level);
            delete_language(index);
        }
    };
    view! {
        <RenderTab  no=7 active_page=select_tab>    
        <Show when=move || select_tab() == 7>
        <Suspense fallback=move || view! { <p>"LoadingIntro..."</p> }> 
        <div class="editContainer">
        <h1>"Edit Language"</h1>             
        <div class="formRow">   
            <InputField input_type="text" id="language_name" validation=validate_language label="Language" set_value=set_language_name  get_value=language_name require=true />        
            <div class="formGroup">
                <label id="language_level">"Level"</label>
                <select
                class="selectDropdown"
                    id="language_level"
                    prop:value=language_level
                    on:change=move |ev| {
                        set_language_level(event_target_value(&ev));
                    }>
                    <For
                    each=move || LANGUAGE_LEVELS.to_vec()
                    key=|&(value, _)| value
                    children=move |(value, label)| {
                        view! {
                            <option value=value>{label}</option>
                        }
                    }
                />
                </select>
                <button
                type="button"
                    class="addButton"
                on:click=add_language >
                "Add Language"
            </button>
            </div>
        </div>
        <LanguageChips
        languages=languages
        on_delete=Callback::new(move |index| delete_language(index))
        on_edit=Callback::new(move |index| edit_language(index))
       is_edit=true />
    </div>
    </Suspense>
    </Show>
        </RenderTab>
    }
}
