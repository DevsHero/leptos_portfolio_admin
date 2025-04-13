use leptos::*;

use crate::app::components::agents::chat::ChatComponent;
#[component]
pub fn ChatPage() -> impl IntoView {
    view! {
        // Your existing UI
        <div class="chat-wrapper">
            <h2>"AI Agent Chat"</h2>
            <ChatComponent />
        </div>
    }
}
