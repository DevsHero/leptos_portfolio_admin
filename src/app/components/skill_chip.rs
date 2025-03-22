use crate::app::models::Skill;
use leptos::{ either::Either, prelude::* };
use leptos_icons::Icon;
use icondata as i;
#[component]
pub fn SkillChips(
    skills: ReadSignal<Vec<Skill>>,
    #[prop(optional)] on_delete: Option<Callback<usize>>,
    #[prop(optional)] on_edit: Option<Callback<usize>>,
    is_edit: bool
) -> impl IntoView {
    view! {
        <div class="skillList">
            {move || skills.get().into_iter().enumerate().map(|(index, skill)| {
                let level_class = skill.level.to_lowercase();
                view! {
                    <div class="skillChip">
                        <span class="skillName">{skill.name}</span>
                        <span class=format!("levelBadge level-{}", level_class)>
                            {skill.level}
                        </span>
                        {

                            view! {
                                <>
                                    {if is_edit {
                                      Either::Left( view! {
                                            <div class="inputArrayRow">
                                        <button
                                        type="button" 
                                            class="editButton"
                                            style="margin-right:10px;"
                                            on:click=move |_| {
                                                if let Some(ref callback) = on_edit {
                                                     (callback, index);
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
                                                  (callback, index);
                                            }
                                        }
                                    >
                                    <Icon icon={i::BsTrash} />
                                    </button>
                                    </div>
                                        })
                                    } else {
                                        Either::Right(())
                                    }}
                                </>
                            }
                        }
                    </div>
                }
            }).collect_view() }
        </div>
    }
}
