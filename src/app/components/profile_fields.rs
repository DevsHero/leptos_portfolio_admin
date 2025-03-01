use crate::app::models::Skill;
use leptos::{ prelude::* };
#[component]
pub fn ProfileFields(
    skills: ReadSignal<Vec<Skill>>, // Change to ReadSignal
    on_delete: Callback<usize>,
    use_delete: bool
) -> impl IntoView {
    view! {
        <div class="skillList">
            {move || skills.get().into_iter().enumerate().map(|(index, skill)| {
                view! {
                    <div class="skillChip">
                        <span class="skillName">{&skill.name}</span>
                        <span class=format!("levelBadge level-{}", skill.level.to_lowercase())>
                            {&skill.level}
                        </span>
                        {if use_delete {
                            view! { <div>   <button
                            class="deleteSkill"
                            on:click=move |_| on_delete.call(index)
                        >
                            "×"
                        </button> </div> }
                        } else{
                            view! {  <div> </div>  }
                        }}
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
