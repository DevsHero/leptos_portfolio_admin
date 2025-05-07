use leptos::*;
use crate::app::components::{ agents::chat::ChatComponent, layouts::Topbar };

#[component]
pub fn ChatPage() -> impl IntoView {
    let (is_ready, set_is_ready) = create_signal(false);
    create_effect(move |_| {
        set_is_ready.set(true);
    });

    view! {
        <Suspense fallback=move || view! { <p>"Loading Chat..."</p> }>
        <Topbar is_ready=is_ready/>
        <div class="chat-wrapper">
            <b><h1 style="font-size: 1.2rem; text-align:center; margin-bottom: 0.5rem;">"Chat With AI"</h1></b>
                <ChatComponent />
           
        </div>
        </Suspense>
    }
}
