use crate::app::{ constants::constant::SKILL_LEVELS, models::Skill };
use leptos::*;
use leptos_icons::Icon;
use icondata as i;
#[component]
pub fn SkillRecords(
    skills: ReadSignal<Vec<Skill>>,
    #[prop(optional)] on_delete: Option<Callback<usize>>,
    #[prop(optional)] on_edit: Option<Callback<usize>>,
    is_edit: bool
) -> impl IntoView {
    view! {
        <div class="skillList" style={if is_edit {"height:100%"} else {""}}>
            {move || skills.get().into_iter().enumerate().map(|(index, skill)| {
                let level_info = SKILL_LEVELS
                .iter()
                .find(|&&(value, _)| value == skill.level)
                .unwrap_or(&("0", "Unknown"));
                let level_class = skill.level.to_lowercase();
                view! {
                    <div class="skillChip">
                        <span class="skillName">{skill.name}</span>
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
