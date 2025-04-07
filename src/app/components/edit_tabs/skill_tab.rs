use crate::app::components::{
    show_error_toast,
    show_success_toast,
    InputField,
    SkillChips,
    RenderTab,
};

use crate::app::constants::constant::SKILL_LEVELS;
use crate::app::models::Skill;
use leptos::*;

#[component]
pub fn EditSkillTab(
    skills: ReadSignal<Vec<Skill>>,
    set_skills: WriteSignal<Vec<Skill>>,
    set_is_update_skill: WriteSignal<bool>,
    select_tab: ReadSignal<i32>
) -> impl IntoView {
    let (skill_name, set_skill_name) = create_signal(String::new());
    let (skill_level, set_skill_level) = create_signal(String::from("1"));
    let (validate_skill, set_validate_skill) = create_signal(false);

    let add_skill = move |_| {
        set_validate_skill.update(|v| {
            *v = !*v;
        });
        let form_valid = !skill_name.get().trim().is_empty();
        if form_valid {
            let new_skill = Skill {
                name: skill_name.get(),
                level: skill_level.get(),
            };
            set_skills.update(|skills| skills.push(new_skill));
            set_validate_skill.set(false);
            set_skill_name.set(String::new());
            set_skill_level.set(String::from("1"));

            set_is_update_skill(true);

            show_success_toast("Add Skill Success", "Skill Added.");
        } else {
            show_error_toast("Add Skill Failed", "Missing required field.");
        }
    };

    let delete_skill = move |index: usize| {
        set_skills.update(|skills| {
            skills.remove(index);
        });
        set_is_update_skill(true)
    };
    let edit_skill = move |index: usize| {
        let list = skills.get();
        if
            let Some(skill) = list
                .iter()
                .enumerate()
                .find(|(i, _)| *i == index)
        {
            let skill = skill.1.clone();
            set_skill_name.set(skill.name);
            set_skill_level.set(skill.level);
            delete_skill(index);
        }
    };
    view! {
        <RenderTab  no=2 active_page=select_tab>    
        <Show when=move || select_tab() == 2>
        <Suspense fallback=move || view! { <p>"LoadingIntro..."</p> }> 
        <div class="editContainer">
        <h1>"Edit Skill"</h1>             
        <div class="formRow">   
            <InputField input_type="text" id="skill_name" validation=validate_skill label="Skill Name" set_value=set_skill_name  get_value=skill_name require=true />        
            <div class="formGroup">
                <label id="skill_level">"Level"</label>
                <select
                class="selectDropdown"
                id="skill_level"
                prop:value=skill_level
                on:change=move |ev| {
                    set_skill_level(event_target_value(&ev));
                }
            >
                <For
                    each=move || SKILL_LEVELS.to_vec()
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
                on:click=add_skill >
                "Add Skill"
            </button>
            </div>
        </div>
        <SkillChips
        skills=skills
        on_delete=Callback::new(move |index| delete_skill(index))
        on_edit=Callback::new(move |index| edit_skill(index))
       is_edit=true />
          </div>
          </Suspense>
          </Show>
          </RenderTab>
    }
}
