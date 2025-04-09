use leptos::{ component, view, IntoView };

use crate::app::constants::constant::{ INTRO_LOGO_URL, INTRO_MAIN_TEXT, INTRO_SUB_TEXT };

#[component]
pub fn Intro() -> impl IntoView {
    view! {
        <div class="loading flex flex-col items-center justify-center min-h-screen space-y-4 relative overflow-hidden">
            <div class="absolute inset-0 bg-white opacity-0 animate-flash">  
            <img
            alt="loading"
            src=   {INTRO_LOGO_URL}
            class="logo-animate animate-spin-slow "
        />
            <p class="text-gray-600 animate-fade-in-up delay-100 typing-text">
            {INTRO_MAIN_TEXT}
            </p>
            <div class="animate-fade-in-up delay-200">
                <p class="text-sm text-gray-500 typing-subtext">
                {INTRO_SUB_TEXT}
                </p>
              
            </div></div>
        </div>
    }
}
