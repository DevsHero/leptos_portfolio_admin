use leptos::*;
use leptos::logging::log;
use wasm_bindgen::JsValue;
use web_sys::{ js_sys::{ self, Uint8Array }, Blob, BlobPropertyBag, Url };
use leptos_icons::Icon;
use icondata as i;
use crate::app::server::api::pdf_export;
use crate::app::models::Profile;
use base64::{ engine::general_purpose::STANDARD, Engine as _ };

#[component]
pub fn PdfExportButton(profile: Profile) -> impl IntoView {
    let export_action = create_action(move |profile_to_export: &Profile| {
        let profile_clone = profile_to_export.clone();
        async move {
            log!("Calling pdf_export server function...");
            pdf_export(profile_clone).await
        }
    });
    let is_generating = create_rw_signal(false);
    create_effect(move |_| {
        if let Some(result) = export_action.value().get() {
            match result {
                Ok(encoded_pdf) => {
                    log!("PDF data received from action, processing...");
                    // --- Decoding/Blob/URL logic ---
                    match STANDARD.decode(encoded_pdf) {
                        Ok(decoded_pdf) => {
                            let parts = js_sys::Array::new();
                            parts.push(&JsValue::from(Uint8Array::from(decoded_pdf.as_slice())));
                            let options = BlobPropertyBag::new();
                            options.set_type("application/pdf");
                            match Blob::new_with_u8_array_sequence_and_options(&parts, &options) {
                                Ok(blob) => {
                                    match Url::create_object_url_with_blob(&blob) {
                                        Ok(url) => {
                                            if let Some(window) = web_sys::window() {
                                                match
                                                    window.open_with_url_and_target(&url, "_blank")
                                                {
                                                    Ok(Some(_)) => {
                                                        // Don't need to focus usually
                                                        log!("Opened PDF in new tab.");
                                                        // Clean up the object URL after a short delay
                                                        let url_clone = url.clone();
                                                        set_timeout(move || {
                                                            if
                                                                let Err(e) = Url::revoke_object_url(
                                                                    &url_clone
                                                                )
                                                            {
                                                                log!(
                                                                    "Failed to revoke Object URL: {:?}",
                                                                    e
                                                                );
                                                            } else {
                                                                log!("Revoked Object URL");
                                                            }
                                                        }, std::time::Duration::from_secs(10)); // Adjust delay as needed
                                                    }
                                                    Ok(None) =>
                                                        log!("Browser blocked opening new tab."), // More likely scenario
                                                    Err(e) =>
                                                        log!("Error opening new tab: {:?}", e),
                                                }
                                            }
                                            // No Url::revoke_object_url here, do it after opening
                                        }
                                        Err(e) => log!("Error creating object URL: {:?}", e),
                                    }
                                }
                                Err(e) => log!("Error creating Blob: {:?}", e),
                            }
                        }
                        Err(e) => log!("Error decoding base64 PDF data: {:?}", e),
                    }
                    is_generating.set(false); // Set loading false on success
                }
                Err(e) => {
                    // Handle the error from pdf_export
                    log!("Error fetching PDF data via action: {:?}", e);
                    is_generating.set(false); // Set loading false on error
                    // Optionally display an error message to the user
                }
            }
            // Reset the action value so the effect doesn't re-run with old data if something else triggers it
            export_action.value().set(None);
        }
    });

    // --- Click Handler ---
    let handler = move |_| {
        // Check if we should generate or use the direct link
        if profile.pdf.use_generate {
            // Assuming `profile.pdf.use_generate` tells us which mode
            log!("Generate button clicked.");
            // Prevent multiple clicks while generating
            if is_generating.get() {
                log!("Already generating PDF, ignoring click.");
                return;
            }
            is_generating.set(true); // Set loading state
            // Dispatch the action, passing the current profile data
            export_action.dispatch(profile.clone());
        } else {
            // Open the direct PDF link
            log!("Direct link button clicked.");
            if let Some(link) = &profile.pdf.pdf_link {
                // Check if link exists
                if let Some(window) = web_sys::window() {
                    if let Err(e) = window.open_with_url_and_target(link, "_blank") {
                        log!("Could not open direct link tab: {:?}", e);
                    }
                }
            } else {
                log!("No direct PDF link available.");
            }
        }
    };

    view! {
        <button
            type="button"
            class="pdfIcon"
            prop:disabled=is_generating 
            on:click=handler
        >
           { move || if is_generating.get() {
                view! { <Icon icon=i::FaSpinnerSolid class="logo-animate" />} // Example loading spinner
           } else {
                view! { <Icon icon=i::FaFilePdfRegular /> }
           }}
        </button>
    }
}
