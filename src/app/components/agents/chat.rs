use leptos::*;
use leptos_icons::*;
use serde::{ Deserialize, Serialize };
use wasm_bindgen::{ prelude::*, JsCast };
use web_sys::{ js_sys, BinaryType, CloseEvent, Element, ErrorEvent, MessageEvent, WebSocket };

// Message types to communicate with the AI agent
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientMessage {
    Chat {
        content: String,
    },
    ModifyCss {
        selector: String,
        property: String,
        value: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    Response {
        content: String,
    },
    CssModified {
        selector: String,
        property: String,
        value: String,
    },
    Error {
        message: String,
    },
}

// A single chat message (for UI)
#[derive(Clone, Debug)]
struct ChatMessage {
    content: String,
    is_user: bool,
}

// Main chat component
#[component]
pub fn ChatComponent() -> impl IntoView {
    // State for the chat interface
    let (messages, set_messages) = create_signal(Vec::<ChatMessage>::new());
    let (input_text, set_input_text) = create_signal(String::new());
    let (is_connected, set_is_connected) = create_signal(false);
    let (is_error, set_is_error) = create_signal(false);
    let (error_message, set_error_message) = create_signal(String::new());

    // Create a websocket connection using StoredValue instead of NodeRef
    let ws_stored = create_local_resource(
        || (),
        move |_| async move {
            // Initialize WebSocket connection
            let ws_url = format!(
                "ws://localhost:4000" // Change to match your agent server address
                // Remove the /api/ws path or adjust to match your server's path
            );

            match WebSocket::new(&ws_url) {
                Ok(ws) => {
                    ws.set_binary_type(BinaryType::Arraybuffer);
                    Some(ws)
                }
                Err(err) => {
                    set_is_error.set(true);
                    set_error_message.set(format!("WebSocket connection error: {:?}", err));
                    logging::log!("Failed to create WebSocket: {:?}", err);
                    None
                }
            }
        }
    );

    // Set up event handlers when the WebSocket is available
    create_effect(move |_| {
        if let Some(Some(ws)) = ws_stored.get() {
            // Set up event handlers
            let on_open = Closure::wrap(
                Box::new(move |_| {
                    set_is_connected.set(true);
                    set_is_error.set(false);
                    logging::log!("WebSocket connection opened");
                }) as Box<dyn FnMut(JsValue)>
            );

            let set_messages_clone = set_messages.clone();
            let on_message = Closure::wrap(
                Box::new(move |e: MessageEvent| {
                    if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
                        let text_string = String::from(text);

                        // Parse the server message
                        match serde_json::from_str::<ServerMessage>(&text_string) {
                            Ok(ServerMessage::Response { content }) => {
                                // Add the AI's response to the chat
                                set_messages_clone.update(|messages| {
                                    messages.push(ChatMessage {
                                        content,
                                        is_user: false,
                                    });
                                });
                            }
                            Ok(ServerMessage::CssModified { selector, property, value }) => {
                                logging::log!("Modifying CSS: {} {} {}", selector, property, value);

                                // Apply CSS modification
                                if
                                    let Ok(elements) = window()
                                        .document()
                                        .unwrap()
                                        .query_selector_all(&selector)
                                {
                                    for i in 0..elements.length() {
                                        if let Some(element) = elements.get(i) {
                                            if
                                                let Some(element_cast) =
                                                    element.dyn_ref::<Element>()
                                            {
                                                // Correct way to set style property
                                                let _ = js_sys::Reflect::set(
                                                    &element_cast
                                                        .unchecked_ref::<web_sys::HtmlElement>()
                                                        .style(),
                                                    &JsValue::from_str(&property),
                                                    &JsValue::from_str(&value)
                                                );
                                            }
                                        }
                                    }
                                }

                                // Notify in chat about the CSS change
                                set_messages_clone.update(|messages| {
                                    messages.push(ChatMessage {
                                        content: format!(
                                            "(Modified CSS: {} {} {})",
                                            selector,
                                            property,
                                            value
                                        ),
                                        is_user: false,
                                    });
                                });
                            }
                            Ok(ServerMessage::Error { message }) => {
                                // Show error in chat
                                set_messages_clone.update(|messages| {
                                    messages.push(ChatMessage {
                                        content: format!("Error: {}", message),
                                        is_user: false,
                                    });
                                });
                            }
                            Err(e) => {
                                logging::log!("Failed to parse message: {:?}", e);
                            }
                        }
                    }
                }) as Box<dyn FnMut(MessageEvent)>
            );

            let set_is_connected_clone = set_is_connected.clone();
            let set_is_error_clone = set_is_error.clone();
            let set_error_message_clone = set_error_message.clone();
            let on_close = Closure::wrap(
                Box::new(move |e: CloseEvent| {
                    set_is_connected_clone.set(false);
                    set_is_error_clone.set(true);
                    set_error_message_clone.set(
                        format!("WebSocket closed: {} ({})", e.reason(), e.code())
                    );
                    logging::log!("WebSocket closed: {} ({})", e.reason(), e.code());
                }) as Box<dyn FnMut(CloseEvent)>
            );

            let set_is_error_clone = set_is_error.clone();
            let set_error_message_clone = set_error_message.clone();
            let on_error = Closure::wrap(
                Box::new(move |e: ErrorEvent| {
                    set_is_error_clone.set(true);
                    set_error_message_clone.set(format!("WebSocket error: {}", e.message()));
                    logging::log!("WebSocket error: {}", e.message());
                }) as Box<dyn FnMut(ErrorEvent)>
            );

            ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
            ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
            ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));
            ws.set_onerror(Some(on_error.as_ref().unchecked_ref()));

            // Store closures to prevent garbage collection
            on_open.forget();
            on_message.forget();
            on_close.forget();
            on_error.forget();
        }
    });

    // Function to send a message
    let send_message = move |_| {
        let input_text_value = input_text.get();
        if input_text_value.trim().is_empty() {
            return;
        }

        // Add the user's message to the chat
        set_messages.update(|messages| {
            messages.push(ChatMessage {
                content: input_text_value.clone(),
                is_user: true,
            });
        });

        // Send the message via WebSocket
        if let Some(Some(ws)) = ws_stored.get() {
            if ws.ready_state() == 1 {
                // OPEN
                let client_msg = ClientMessage::Chat { content: input_text_value };
                let json = serde_json::to_string(&client_msg).unwrap_or_else(|e| {
                    logging::log!("Failed to serialize message: {:?}", e);
                    "{}".to_string()
                });
                let _ = ws.send_with_str(&json);
            } else {
                set_is_error.set(true);
                set_error_message.set("WebSocket is not connected".to_string());
            }
        }

        // Clear input field
        set_input_text.set("".to_string());
    };

    // Handle enter key press
    let on_keypress = move |ev: ev::KeyboardEvent| {
        if ev.key() == "Enter" && !ev.shift_key() {
            ev.prevent_default();
            send_message(ev::MouseEvent::new("click").unwrap());
        }
    };

    view! {
        <div class="chat-container">
            <div class="chat-messages" id="chat-messages">
                {move || messages.get().into_iter().map(|msg| {
                    let is_user = msg.is_user;
                    view! {
                        <div class={if is_user { "message user-message" } else { "message agent-message" }}>
                            <div class="message-content">{msg.content}</div>
                        </div>
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
                        "Connected"
                    } else {
                        "Connecting..."
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
                />
                <button
                    class="chat-send-button"
                    on:click=send_message
                    disabled=move || !is_connected.get() || input_text.get().trim().is_empty()
                >
                    <Icon icon=icondata::ChPaperPlane />
                </button>
            </div>
        </div>
    }
}
