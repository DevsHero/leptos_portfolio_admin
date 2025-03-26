use leptos::*;
use leptos::logging::log;
use wasm_bindgen::JsValue;
use web_sys::{ js_sys::{ self, Uint8Array }, Blob, BlobPropertyBag, Url };
use leptos_icons::Icon;
use icondata as i;
use crate::app::server::api::pdf_export;

#[component]
pub fn PdfExportButton() -> impl IntoView {
    let pdf_data = create_resource(
        || (), // No input needed for this server function
        |_| async move {
            match pdf_export().await {
                Ok(data) => Some(data),
                Err(e) => {
                    log!("Error fetching PDF data: {:?}", e);
                    None
                }
            }
        }
    );

    let open_pdf = move |_| {
        if let Some(Some(encoded_pdf)) = pdf_data.get() {
            // Decode the base64 string
            match base64::decode(encoded_pdf) {
                Ok(decoded_pdf) => {
                    // Create a Blob from the decoded bytes
                    let parts = js_sys::Array::new();
                    parts.push(&JsValue::from(Uint8Array::from(decoded_pdf.as_slice())));
                    let options = BlobPropertyBag::new();
                    options.set_type("application/pdf");
                    match Blob::new_with_u8_array_sequence_and_options(&parts, &options) {
                        Ok(blob) => {
                            // Create a URL for the Blob
                            match Url::create_object_url_with_blob(&blob) {
                                Ok(url) => {
                                    // Open the URL in a new tab
                                    if let Some(window) = web_sys::window() {
                                        match window.open_with_url_and_target(&url, "_blank") {
                                            Ok(Some(new_window)) => {
                                                match new_window.focus() {
                                                    Ok(_) => log!("Opened PDF in new tab."),
                                                    Err(e) =>
                                                        log!("Failed to focus new tab: {:?}", e),
                                                }
                                            }
                                            Ok(None) => log!("Failed to open new tab."),
                                            Err(e) => log!("Error opening new tab: {:?}", e),
                                        }
                                    }
                                    // Optionally revoke the object URL after opening
                                    // Url::revoke_object_url(&url).unwrap();
                                }
                                Err(e) => log!("Error creating object URL: {:?}", e),
                            }
                        }
                        Err(e) => log!("Error creating Blob: {:?}", e),
                    }
                }
                Err(e) => log!("Error decoding base64 PDF data: {:?}", e),
            }
        } else {
            log!("PDF data not yet loaded or an error occurred.");
        }
    };

    view! {
        <button 
        class="pdfIcon" 
        on:click=open_pdf>
        <Icon icon={i::FaFilePdfRegular} />
        </button>
       
    }
}
