use leptos::*;
use leptos_toaster::{ Theme, Toast, ToastId, ToastOptions, ToastVariant, ToasterPosition, Toasts };

pub fn show_toast(title: View, detail: View, variant: ToastVariant) {
    let toast_context = match use_context::<Toasts>() {
        Some(ctx) => ctx,
        None => {
            leptos::logging::error!(
                "Toasts context not found. Ensure <Toasts /> is rendered in the layout."
            );
            return;
        }
    };

    let toast_id = ToastId::new();
    toast_context.toast(
        view! {
            <Toast
                toast_id=toast_id 
                variant=variant
                theme=Theme::Dark 
                title=title
                description=Some(detail)
            />
        },
        Some(toast_id),
        Some(ToastOptions {
            dismissible: true,
            duration: Some(std::time::Duration::from_secs(3)),
            position: Some(ToasterPosition::BottomLeft),
            ..Default::default()
        })
    );
}

// Optional: Create specific helpers for common toast types
pub fn show_success_toast(title: &str, detail: &str) {
    show_toast(
        (view! { <p class="toastSuccess">{title.to_string()}</p> }).into_view(),
        detail.to_string().into_view(),
        ToastVariant::Success
    );
}

pub fn show_error_toast(title: &str, detail: &str) {
    show_toast(
        (view! { <p class="toastFail">{title.to_string()}</p> }).into_view(),
        detail.to_string().into_view(),
        ToastVariant::Error
    );
}

pub fn show_info_toast(title: &str, detail: &str) {
    show_toast(
        (view! { <p class="toastInfo">{title.to_string()}</p> }).into_view(),
        detail.to_string().into_view(),
        ToastVariant::Info // Or another appropriate variant like Normal
    );
}
