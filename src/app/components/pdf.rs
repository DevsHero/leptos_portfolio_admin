use leptos::*;
use wasm_bindgen::JsValue;
use web_sys::{ js_sys::{ self, Uint8Array }, Blob, BlobPropertyBag, Url };
use leptos_icons::Icon;
use icondata as i;
use crate::app::server::api::{ check_pdf_exits_api, get_pdf_file_api, pdf_create_api };
use crate::app::models::Profile;
use base64::{ engine::general_purpose::STANDARD, Engine as _ };

#[component]
pub fn PdfExportButton(profile: Profile) -> impl IntoView {
    let profile_for_handler = profile.clone();
    let is_generating = create_rw_signal(false);
    let open_new_tab = async move |encoded_pdf: String| {
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
                                    match window.open_with_url_and_target(&url, "_blank") {
                                        Ok(Some(_)) => {
                                            let url_clone = url.clone();
                                            set_timeout(move || {
                                                if let Err(e) = Url::revoke_object_url(&url_clone) {
                                                    println!(
                                                        "Failed to revoke Object URL: {:?}",
                                                        e
                                                    );
                                                } else {
                                                    println!("Revoked Object URL");
                                                }
                                            }, std::time::Duration::from_secs(10)); // Adjust delay for delete blob
                                        }
                                        Ok(None) => println!("Browser blocked opening new tab."),
                                        Err(e) => println!("Error opening new tab: {:?}", e),
                                    }
                                }
                            }
                            Err(e) => println!("Error creating object URL: {:?}", e),
                        }
                    }
                    Err(e) => println!("Error creating Blob: {:?}", e),
                }
            }
            Err(e) => println!("Error decoding base64 PDF data: {:?}", e),
        }
        is_generating.set(false); // Set loading false on success
    };

    let handler = {
        move |_| {
            let profile = profile_for_handler.clone();
            spawn_local(async move {
                if profile.pdf.use_generate {
                    if is_generating.get() {
                        return;
                    }
                    is_generating.set(true);
                    let check_pdf_exits_api = check_pdf_exits_api().await;
                    if !check_pdf_exits_api.unwrap() {
                        let create_pdf = pdf_create_api(profile.clone()).await;
                        open_new_tab(create_pdf.unwrap()).await
                    } else {
                        let get_pdf_file_api = get_pdf_file_api().await;
                        open_new_tab(get_pdf_file_api.unwrap()).await
                    }
                } else {
                    if let Some(link) = &profile.pdf.pdf_link {
                        if let Some(window) = web_sys::window() {
                            if let Err(e) = window.open_with_url_and_target(link, "_blank") {
                                println!("Could not open direct link tab: {:?}", e);
                            }
                        }
                    } else {
                        println!("No direct PDF link available.");
                    }
                }
            });
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
