use crate::app::models::Skill;
use leptos::*;
use leptos_icons::Icon;
use icondata as i;
#[component]
pub fn SkillChips(
    skills: ReadSignal<Vec<Skill>>,
    on_delete: Option<Callback<usize>>, // made optional
    is_page: bool,
    use_delete: bool
) -> impl IntoView {
    view! {
        <div class=if is_page {"skillPageList"} else {"skillList"}>
            {move || skills.get().into_iter().enumerate().map(|(index, skill)| {
                let level_class = skill.level.to_lowercase();
                view! {
                    <div class="skillChip">
                        <span class="skillName">{skill.name}</span>
                        <span class=format!("levelBadge level-{}", level_class)>
                            {skill.level}
                        </span>
                        {
                            // Wrap the conditional in a fragment so both branches return the same type.
                            view! {
                                <>
                                    {if use_delete {
                                        view! {
                                            <div>
                                                <button
                                                    class="deleteButton"
                                                    on:click=move |_| {
                                                        if let Some(ref callback) = on_delete {
                                                            callback.call(index);
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
