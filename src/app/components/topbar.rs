use leptos::prelude::*;
use leptos_icons::Icon;
use icondata as i;
use crate::app::components::ThemeButton;

#[component]
pub fn Topbar() -> impl IntoView {
    (
        view! {
        <section class="topbar">
            <div class="pill">
                <a  href="/" class="topbarButton ">
                    <Icon icon={i::AiHomeOutlined} />
                </a>
                <a
                    href="/edit"  // Client-side navigation
                    class="topbarButton"
                >
                    <Icon icon={i::OcGearSm} />
                </a>
                <ThemeButton />
            </div>
        </section>
    }
    ).into_any()
}
