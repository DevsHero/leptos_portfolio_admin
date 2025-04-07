use leptos::*;
use leptos_toaster::{ Theme, Toast, ToastId, ToastOptions, ToastVariant, ToasterPosition, Toasts };
#[derive(Clone)]
pub struct ToastManager {
    pub create_toast: Box<dyn Fn(View, View, ToastVariant) + 'static>,
}

// 2. Create a context provider component
#[component]
pub fn ToastProvider(children: Children) -> impl IntoView {
    let toasts = expect_context::<Toasts>();

    let create_toast = Box::new(move |title: View, detail: View, variant: ToastVariant| {
        let toast_id = ToastId::new();
        toasts.toast(
            view! {
                <Toast
                    toast_id
                    variant=variant
                    theme=Theme::Dark
                    invert=false
                    rich_colors=false
                    title=view! { {title} }.into_view()
                    description=Some(view! { {detail} }.into_view())
                />
            },
            Some(toast_id),
            Some(ToastOptions {
                dismissible: true,
                duration: Some(std::time::Duration::from_secs(4)),
                position: Some(ToasterPosition::BottomLeft),
            })
        );
    });

    provide_context(ToastManager { create_toast });

    children()
}

// 3. Create a hook for easy access
pub fn use_toast() -> ToastManager {
    expect_context::<ToastManager>()
}
