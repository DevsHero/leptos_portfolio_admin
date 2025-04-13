use leptos::*;
use leptos_router::A;
use leptos_icons::Icon;
use icondata as i;

use crate::app::components::layouts::ThemeButton;

#[component]
pub fn Topbar(is_ready: ReadSignal<bool>) -> impl IntoView {
    view! {
        <section class="topbar">
         {move || view! {   <div class= if !is_ready.get() { "loadingPill " } else { "pill" } >
         <A  href="/" class="topbarButton ">
             <Icon icon={i::AiHomeOutlined} />
         </A>
         <A
             href="/edit"   
             class="topbarButton"
         >
             <Icon icon={i::BiEditSolid} />
         </A> 
         <A
         href="/chat"   
         class="topbarButton"
     >
         <Icon icon={i::MdiFaceAgent} />
     </A> 
         <ThemeButton />
     </div>}}
        </section>
    }
}
