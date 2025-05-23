use leptos::*;
use leptos_icons::*;
use serde::{ Deserialize, Serialize };
use wasm_bindgen::{ prelude::*, JsCast };
use web_sys::{ js_sys, BinaryType, CloseEvent, ErrorEvent, MessageEvent, WebSocket, Request, RequestInit, Response };
use wasm_bindgen_futures::JsFuture;
use chrono::{ Local, TimeZone };
use crate::app::models::server::WSSignedConfig;
use crate::app::server::api::get_ws_signed_config_api;
use crate::app::utils::utils::get_icon_by_name;
use serde_wasm_bindgen;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ClientMessage {
    #[serde(rename = "chat")] Chat {
        content: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ServerMessage {
    #[serde(rename = "response")] Response {
        content: String,
        timestamp: i64,
    },
    #[serde(rename = "error")] Error {
        message: String,
    },
    #[serde(rename = "processing")]
    Processing,
}

#[derive(Clone, Debug)]
struct ChatMessage {
    content: String,
    is_user: bool,
    is_processing: bool,
    timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SuggestionGroup {
    group_title: String,
    icon: String,
    suggestions: Vec<String>,
}

 

fn format_timestamp(timestamp: i64) -> String {
    match Local.timestamp_opt(timestamp, 0) {
        chrono::LocalResult::Single(dt) => dt.format("%Y-%m-%d %H:%M").to_string(),
        _ => "Invalid time".to_string(),
    }
}

#[component]
pub fn ChatComponent() -> impl IntoView {
    let (messages, set_messages) = create_signal(Vec::<ChatMessage>::new());
    let (input_text, set_input_text) = create_signal(String::new());
    let (is_connected, set_is_connected) = create_signal(false);
    let (is_error, set_is_error) = create_signal(false);
    let (error_message, set_error_message) = create_signal(String::new());
    let (is_processing, set_is_processing) = create_signal(false);
    let (suggestion_groups, set_suggestion_groups) = create_signal(Vec::<SuggestionGroup>::new());
    
    let ws_cfg = create_resource(
        || (),
        |_| async move { get_ws_signed_config_api().await }
    );
    
    let suggestions_resource = create_resource(
        || (),
        |_| async move {
            match fetch_suggestions().await {
                Ok(data) => {
                    logging::log!("Loaded suggestions: {:?}", data);
                    data
                }
                Err(e) => {
                    logging::error!("Failed to load suggestions: {:?}", e);
                    Vec::new()
                }
            }
        }
    );

    create_effect(move |_| {
        if let Some(loaded_groups) = suggestions_resource.get() {
            set_suggestion_groups.set(loaded_groups);
        }
    });
    
    let ws_stored = create_local_resource(
        move || ws_cfg.get().and_then(|result| result.ok()),
        move |maybe_signed: Option<WSSignedConfig>| async move {
            if let Some(signed) = maybe_signed {
                let url = format!(
                    "{}?X-Api-Ts={}&X-Api-Sign={}",
                    signed.host, signed.ts, signed.sig
                );
                logging::log!("Connecting to WebSocket: {}", url);
                match WebSocket::new(&url) {
                    Ok(ws) => { ws.set_binary_type(BinaryType::Arraybuffer); Some(ws) }
                    Err(err) => {
                        logging::error!("WS new failed: {:?}", err);
                        None
                    }
                }
            } else {
                None
            }
        },
    );

    create_effect(move |_| {
        if let Some(Some(ws)) = ws_stored.get() {
            let on_open = Closure::wrap(
                Box::new(move |_| {
                    set_is_connected.set(true);
                    set_is_error.set(false);
                    logging::log!("WebSocket connection opened");
                }) as Box<dyn FnMut(JsValue)>
            );

         
          
            let on_message = Closure::wrap(
                Box::new(move |e: MessageEvent| {
                    if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
                        let text_string = String::from(text);

                        match serde_json::from_str::<ServerMessage>(&text_string) {
                            Ok(ServerMessage::Response { content, timestamp }) => {
                                set_is_processing.set(false);
                                set_messages.update(|messages| {
                                    if let Some(last) = messages.last_mut() {
                                        if last.is_processing {
                                            messages.pop();
                                        }
                                    }
                                    messages.push(ChatMessage {
                                        content,
                                        is_user: false,
                                        is_processing: false,
                                        timestamp: Some(timestamp),
                                    });
                                });
                            }
                            Ok(ServerMessage::Processing) => {
                                set_is_processing.set(true);
                                set_messages.update(|messages| {
                                    if let Some(last) = messages.last_mut() {
                                        if last.is_processing {
                                            messages.pop();
                                        }
                                    }
                                    messages.push(ChatMessage {
                                        content: "".to_string(),
                                        is_user: false,
                                        is_processing: true,
                                        timestamp: None,
                                    });
                                });
                                logging::log!("Server is processing...");
                            }
                            Ok(ServerMessage::Error { message }) => {
                                set_is_processing.set(false);
                                set_messages.update(|messages| {
                                    if let Some(last) = messages.last_mut() {
                                        if last.is_processing {
                                            messages.pop();
                                        }
                                    }
                                    messages.push(ChatMessage {
                                        content: format!("Error: {message}"  ),
                                        is_user: false,
                                        is_processing: false,
                                        timestamp: None,
                                    });
                                });
                            }
                            Err(e) => {
                                set_is_processing.set(false);
                                logging::log!("Failed to parse message: {:?}", e);

                                set_messages.update(|messages| {
                                    messages.push(ChatMessage {
                                        content: format!("Error parsing server message."),
                                        is_user: false,
                                        is_processing: false,
                                        timestamp: None,
                                    });
                                });
                            }
                        }
                    }
                }) as Box<dyn FnMut(MessageEvent)>
            );

            let set_is_connected_clone = set_is_connected.clone();
            let set_is_error_clone = set_is_error.clone();
            let set_error_message_clone = set_error_message.clone();
            let set_is_processing_close = set_is_processing.clone();
            let on_close = Closure::wrap(
                Box::new(move |e: CloseEvent| {
                    set_is_connected_clone.set(false);
                    set_is_error_clone.set(true);
                    set_is_processing_close.set(false);
                    set_error_message_clone.set(
                        format!("WebSocket closed: {} ({})", e.reason(), e.code())
                    );
                    logging::log!("WebSocket closed: {} ({})", e.reason(), e.code());
                }) as Box<dyn FnMut(CloseEvent)>
            );

            let set_is_error_clone = set_is_error.clone();
            let set_error_message_clone = set_error_message.clone();
            let set_is_processing_error = set_is_processing.clone();
            let on_error = Closure::wrap(
                Box::new(move |e: ErrorEvent| {
                    set_is_error_clone.set(true);
                    set_is_processing_error.set(false);
                    set_error_message_clone.set(format!("WebSocket error: {}", e.message()));
                    logging::log!("WebSocket error: {}", e.message());
                }) as Box<dyn FnMut(ErrorEvent)>
            );

            ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
            ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
            ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));
            ws.set_onerror(Some(on_error.as_ref().unchecked_ref()));

            on_open.forget();
            on_message.forget();
            on_close.forget();
            on_error.forget();
        }
    });

    let send_message = move |_| {
        let input_text_value = input_text.get();
        if input_text_value.trim().is_empty() {
            return;
        }
        let user_timestamp = Local::now().timestamp();

        set_messages.update(|messages| {
            messages.push(ChatMessage {
                content: input_text_value.clone(),
                is_user: true,
                is_processing: false,
                timestamp: Some(user_timestamp),
            });
        });

        if let Some(Some(ws)) = ws_stored.get() {
            if ws.ready_state() == 1 {
                let client_msg = ClientMessage::Chat { content: input_text_value };
                match serde_json::to_string(&client_msg) {
                    Ok(json) => {
                        if let Err(e) = ws.send_with_str(&json) {
                            set_is_error.set(true);
                            set_error_message.set(format!("Error sending message: {:?}", e));
                            logging::log!("Error sending message: {:?}", e);
                        }
                    }
                    Err(e) => {
                        set_is_error.set(true);
                        set_error_message.set(format!("Failed to serialize message: {:?}", e));
                        logging::log!("Failed to serialize message: {:?}", e);
                    }
                }
            } else {
                set_is_error.set(true);
                set_error_message.set("WebSocket is not connected".to_string());
                logging::log!("Attempted to send message, but WebSocket is not connected. State: {}", ws.ready_state());
            }
        } else {
            set_is_error.set(true);
            set_error_message.set("WebSocket is not available to send message.".to_string());
            logging::log!("Attempted to send message, but WebSocket is not available (ws_stored is None or inner None).");
        }

        set_input_text.set("".to_string());
    };

    let on_keypress = move |ev: ev::KeyboardEvent| {
        if ev.key() == "Enter" && !ev.shift_key() {
            ev.prevent_default();

            if is_connected.get() && !input_text.get().trim().is_empty() && !is_processing.get() {
                send_message(ev::MouseEvent::new("click").unwrap());
            }
        }
    };

    view! {
        <div class="chat-container">
            <div class="chat-messages" id="chat-messages">
                {move || {
                    if messages.get().is_empty() && !is_processing.get() && is_connected.get() {
                        view! {
                            <div class="suggestions-center-container">
                                <div class="suggestions-wrapper">
                                    <h3 class="suggestions-title">How can I help you today?</h3>
                                    <div class="suggestions-groups">
                                        {suggestion_groups.get().into_iter().map(|group| {
                                            let icon_data = get_icon_by_name(&group.icon)
                                                .unwrap_or(icondata::BsQuestionCircle);
                                                
                                            view! {
                                                <div class="suggestion-group">
                                                    <div class="suggestion-group-header">
                                                        <Icon icon=icon_data class="suggestion-group-icon" />
                                                        <span class="suggestion-group-title">{group.group_title}</span>
                                                    </div>
                                                    <div class="suggestion-group-items">
                                                        {group.suggestions.into_iter().map(|sugg| {
                                                            let sugg_clone = sugg.clone();
                                                            view! {
                                                                <button
                                                                    class="suggestion-item"
                                                                    on:click=move |_| {
                                                                        set_input_text.set(sugg_clone.clone());
                                                                        if let Some(window) = web_sys::window() {
                                                                            if let Some(document) = window.document() {
                                                                                if let Some(textarea) = document.query_selector(".chat-input").ok().flatten() {
                                                                                    let _ = textarea.dyn_into::<web_sys::HtmlTextAreaElement>()
                                                                                        .map(|el| { let _ = el.focus(); });
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                >
                                                                    {sugg}
                                                                </button>
                                                            }
                                                        }).collect_view()}
                                                    </div>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>
                            </div>
                        }
                    } else {
                        view! { <div></div> }
                    }
                }}

                {move || messages.get().into_iter().map(|msg| {
                    if msg.is_processing {

                        view! {
                            <div class="message agent-message processing-message">
                                <div class="spinner-container">
                                    <div class="spinner"></div>
                                </div>
                            </div>
                        }
                    } else {

                        let outer_cls = if msg.is_user { "message user-message" } else { "message agent-message" };
                        let timestamp_display = msg.timestamp.map(|ts| {
                            view! { <div class="message-timestamp">{format_timestamp(ts)}</div> }
                        });

                        view! {

                            <div class=outer_cls>
                                <div class="message-inner-container">
                                {timestamp_display}
                                    <div class="message-content">{msg.content}</div>
                                </div>
                            </div>
                        }
                    }
                }).collect::<Vec<_>>()}

                {move || {
                    if is_error.get() {
                        view! {
                            <div class="error-message">
                                {error_message.get()}
                            </div>
                        }
                    } else {
                        view! { <div style="display: none;"></div> }
                    }
                }}

                <div class="connection-status">
                    {move || if is_connected.get() {
                        "Connected".to_string()
                    }
                     else {
                        "Not Connected".to_string()
                    }}
                </div>
            </div>

            <div class="chat-input-container">
                <textarea
                    class="chat-input"
                    placeholder="Type your message here..."
                    on:input=move |ev| {
                        set_input_text.set(event_target_value(&ev));
                    }
                    on:keypress=on_keypress
                    prop:value=input_text
                    disabled=move || is_processing.get() || !is_connected.get()
                />
                <button
                    class="chat-send-button"
                    on:click=send_message
                    disabled=move || is_processing.get() || !is_connected.get() || input_text.get().trim().is_empty()
                >
                    <Icon icon=icondata::ChPaperPlane />
                </button>
            </div>
        </div>
    }
}

async fn fetch_suggestions() -> Result<Vec<SuggestionGroup>, JsValue> {
    let   opts = RequestInit::new();
    opts.set_method("GET");
    
    let request = Request::new_with_str_and_init("/assets/pre-suggest.json", &opts)?;
    
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    
    let resp: Response = resp_value.dyn_into()?;
    
    if !resp.ok() {
        return Err(JsValue::from_str(&format!("HTTP error: {}", resp.status())));
    }

    let json = JsFuture::from(resp.json()?).await?;
    
    let groups: Vec<SuggestionGroup> = serde_wasm_bindgen::from_value(json)?;
    Ok(groups)
}
