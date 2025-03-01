use crate::app::models::Skill;
use leptos::{ prelude::* };
#[component]
pub fn ProfileFields(
    skills: ReadSignal<Vec<Skill>>, // Change to ReadSignal
    on_delete: Callback<usize>,
    use_delete: bool
) -> impl IntoView {
    view! {
        <div class="skills-list">
            {move || skills.get().into_iter().enumerate().map(|(index, skill)| {
                view! {
                    <div class="skill-chip">
                        <span class="skill-name">{&skill.name}</span>
                        <span class=format!("level-badge level-{}", skill.level.to_lowercase())>
                            {&skill.level}
                        </span>
                        {if use_delete {
                            view! { <div>   <button
                            class="delete-skill"
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
