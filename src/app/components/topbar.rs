use leptos::*;
use leptos_router::A;
use leptos_icons::Icon;
use icondata as i;

use crate::app::components::ThemeButton;

#[component]
pub fn Topbar() -> impl IntoView {
    view! {
        <section class="topbar">
            <div class="pill">
                <A  href="/" class="topbarButton ">
                    <Icon icon={i::AiHomeOutlined} />
                </A>
                <A
                    href="/edit"  // Client-side navigation
                    class="topbarButton"
                >
                    <Icon icon={i::OcGearSm} />
                </A>
                <ThemeButton />
            </div>
        </section>
    }
}
