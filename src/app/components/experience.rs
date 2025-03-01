use leptos::{ component, view, IntoView };
use crate::app::{ models::portfolio::Experience, utils::convert_date_format };
#[component]
pub fn Experience(experience: Experience, index: String) -> impl IntoView {
    view! {
        <a href=experience.company_url target="_blank" class="experience-container">
            <article class="">
                <span class="experienceRow">
                <span class="experienceRowFirstItem">
                    <img src=experience.company_logo_url alt="Company Icon" />
                    <span class="experienceRowFirstItemText">
                    <h4><b>Company</b>: {experience.company_name}</h4> <h3><b>Position</b>: {experience.position_name}</h3>
                    <p>{convert_date_format(&experience.start_date) } - {convert_date_format(&experience.end_date) }</p>
                    </span>
                    </span>
                    <b><h4 class="experienceNumber">{index} </h4></b>
                </span>
                <div class="descriptions" inner_html=experience.describe></div>
                // <div class="projectIcons">{icns}</div>
            </article>
        </a>
    }
}
