use leptos::{ component, view, IntoView };

#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div class="loading flex flex-col items-center justify-center min-h-screen space-y-4 relative overflow-hidden">
            <div class="absolute inset-0 bg-white opacity-0 animate-flash">  
            <div class="relative" >
                <img
                    alt="loading"
                    src="assets/logo.webp"
                    class="logo-animate animate-spin-slow "
                />
                <div class="absolute inset-0 border-4 border-t-transparent border-blue-500/30 rounded-full animate-spin">
                </div>
            </div>
            <p class="text-gray-600 animate-fade-in-up delay-100 typing-text">
                "Welcome to Hero Portfolio Sites"
            </p>
            <div class="animate-fade-in-up delay-200">
                <p class="text-sm text-gray-500 typing-subtext">
                    "This site is coded in Rust and powered by Leptos."
                </p>
                <div class="mt-2 h-1 bg-gray-200 rounded-full overflow-hidden">
                    <div class="w-1/3 h-full bg-blue-500 animate-progress"></div>
                </div>
            </div></div>
        </div>
    }
}
