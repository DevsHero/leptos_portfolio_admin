use crate::app::{ constants::constant::LANGUAGE_LEVELS, models::Language };
use leptos::*;
use leptos_icons::Icon;
use icondata as i;
#[component]
pub fn LanguageChips(
    languages: ReadSignal<Vec<Language>>,
    #[prop(optional)] on_delete: Option<Callback<usize>>,
    #[prop(optional)] on_edit: Option<Callback<usize>>,
    is_edit: bool
) -> impl IntoView {
    view! {
        <div class="languageList" style={if is_edit {"height:100%"} else {""}}>
            {move || languages.get().into_iter().enumerate().map(|(index, language)| {
                let level_info = LANGUAGE_LEVELS
                .iter()
                .find(|&&(value, _)| value == language.level)
                .unwrap_or(&("0", "Unknown"));
                let level_class = language.level.to_lowercase();
                view! {
                    <div class="skillChip">
                        <span class="skillName">{language.name}</span>
                        <span class=format!("levelBadge level-{}", level_class)>
                            {level_info.1}
                        </span>
                        {

                            view! {
                                <>
                                    {if is_edit {
                                        view! {
                                            <div class="inputArrayRow">
                                        <button
                                        type="button" 
                                            class="editButton"
                                            style="margin-right:10px;"
                                            on:click=move |_| {
                                                if let Some(ref callback) = on_edit {
                                                    leptos::Callable::call(callback, index);
                                                }
                                            }
                                        >
                                        <Icon icon={i::BiEditRegular} />
                                        </button>
                                        <button
                                        type="button" 
                                        class="deleteButton"
                                        on:click=move |_| {
                                            if let Some(ref callback) = on_delete {
                                                leptos::Callable::call(callback, index);
                                            }
                                        }
                                    >
                                    <Icon icon={i::BsTrash} />
                                    </button>
                                    </div>
                                        }
                                    } else {
                                        view! { <div></div> }
                                    }}
                                </>
                            }
                        }
                    </div>
                }
            }).collect::<Vec<_>>() }
        </div>
    }
}
