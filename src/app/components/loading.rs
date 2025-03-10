use leptos::{ component, view, IntoView };

#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div class="loading flex flex-col items-center justify-center p-8">
            <img
          
                alt="loading"
                src="assets/logo.webp"
                width="60"
                height="60"
                class="animate-spin"
            />
            <p class="mt-2">"Hero portfolio site powered by Rust"</p>
            <p>"fetching data..."</p>
        </div>
    }
}
