use leptos::{ component, view, IntoView };
#[component]
pub fn Paypal() -> impl IntoView {
    view! {
        <div class="icon">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32" id="paypal">
                <path
                    fill="var(--text)"
                    d="M26.28 9.58A6.64 6.64 0 0 0 19.65 3H10a1 1 0 0 0-1 .84L5.58 25.29a1 1 0 0 0 .23.8 1 1 0 0 0 .76.36h4.13l-.22 1.39a1 1 0 0 0 1 1.16h4.65a1 1 0 0 0 1-.83l1-6.11h3.15a7.46 7.46 0 0 0 7.43-7.45v-.28a5.87 5.87 0 0 0-2.43-4.75ZM10.85 5h8.8a4.64 4.64 0 0 1 4.51 3.61 5.68 5.68 0 0 0-1.32-.15H14.5a1 1 0 0 0-1 .84l-.59 3.7a1 1 0 0 0 2 .32l.46-2.88h7.49a3.87 3.87 0 0 1 1.4.27 6.47 6.47 0 0 1-6.4 5.69h-4.64a1 1 0 0 0-1 .83L11 24.45H7.74Zm15.86 9.61a5.46 5.46 0 0 1-5.43 5.45h-4a1 1 0 0 0-1 .83l-1 6.11h-2.64l.22-1.39 1.2-7.19h3.78A8.46 8.46 0 0 0 26 12.1a3.82 3.82 0 0 1 .71 2.23Z"
                ></path>
            </svg>
        </div>
    }
}
