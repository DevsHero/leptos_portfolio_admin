use crate::app::models::Skill;
use leptos::*;
#[component]
pub fn SkillChips(
    skills: ReadSignal<Vec<Skill>>,
    on_delete: Callback<usize>,
    use_delete: bool
) -> impl IntoView {
    view! {
        <div class="skills-list">
            {move || skills.get().into_iter().enumerate().map(|(index, skill)| {
                let level_class = skill.level.to_lowercase();
                view! {
                    <div class="skill-chip">
                        <span class="skill-name">{skill.name}</span>
                        <span class=format!("level-badge level-{}", level_class)>
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
                                                    class="delete-skill"
                                                    on:click=move |_| on_delete.call(index)
                                                >
                                                    "×"
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
