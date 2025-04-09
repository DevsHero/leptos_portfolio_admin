use leptos::*;

#[component]
pub fn LockDialog(
    #[prop(into)] is_visible: MaybeSignal<bool>,
    #[prop(default = "Processing...".to_string(), into)] title: String,
    #[prop(
        default = "Please wait while your changes are being processed...".to_string(),
        into
    )] message: String
) -> impl IntoView {
    view! {
        {
            move || (
                if is_visible.get() {
                    view! {
                <div class="lock-overlay">
                    <div class="lock-dialog">
                        <h2>{title.clone()}</h2>
                        <div class="spinner"></div>
                        <p>{message.clone()}</p>
                    </div>
                </div>
            }
                } else {
                    view! { <div></div> }
                }
            )
        }
    }
}
