use crate::types::ChatMessage;
use crate::bindings::processMarkdown;
use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;

pub fn update_ui(messages: &[ChatMessage]) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let chat_container = document.get_element_by_id("chat-messages");
    
    if let Some(container) = chat_container {
        // Очищаем контейнер
        container.set_inner_html("");
        
        for message in messages {
            let role_class = match message.role.as_str() {
                "user" => "user-message",
                "assistant" => "assistant-message",
                _ => "system-message",
            };
            
            let time = js_sys::Date::new(&JsValue::from_f64(message.timestamp));
            let time_str = time.to_locale_time_string("ru-RU");
            
            // Создаем элемент сообщения
            let message_div = document.create_element("div").unwrap();
            message_div.set_class_name(&format!("message {}", role_class));
            
            // Создаем заголовок сообщения
            let header_div = document.create_element("div").unwrap();
            header_div.set_class_name("message-header");
            
            let role_span = document.create_element("span").unwrap();
            role_span.set_class_name("role");
            role_span.set_text_content(Some(match message.role.as_str() {
                "user" => "Вы",
                "assistant" => "Ассистент",
                _ => "Система",
            }));
            
            let time_span = document.create_element("span").unwrap();
            time_span.set_class_name("time");
            time_span.set_text_content(Some(&time_str.as_string().unwrap_or_default()));
            
            header_div.append_child(&role_span).unwrap();
            header_div.append_child(&time_span).unwrap();
            
            // Создаем содержимое сообщения
            let content_div = document.create_element("div").unwrap();
            content_div.set_class_name("message-content");
            
            // Для сообщений ассистента используем Markdown рендеринг
            if message.role == "assistant" && !message.content.is_empty() {
                let markdown_html = processMarkdown(&message.content);
                content_div.set_inner_html(&markdown_html);
            } else {
                content_div.set_text_content(Some(&message.content));
            }
            
            // Собираем элемент сообщения
            message_div.append_child(&header_div).unwrap();
            message_div.append_child(&content_div).unwrap();
            
            // Добавляем в контейнер
            container.append_child(&message_div).unwrap();
        }
        
        // Прокручиваем вниз
        container.set_scroll_top(container.scroll_height());
    }
}

pub fn update_last_message_only(messages: &[ChatMessage]) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let chat_container = document.get_element_by_id("chat-messages");
    
    if let Some(container) = chat_container {
        // Ищем все сообщения
        if let Ok(messages_nodes) = container.query_selector_all(".message") {
            let last_index = messages_nodes.length() - 1;
            
            if let Some(last_node) = messages_nodes.item(last_index) {
                // Преобразуем Node в Element
                if let Some(last_element) = last_node.dyn_ref::<web_sys::Element>() {
                    // Ищем элемент с классом message-content внутри последнего сообщения
                    if let Some(content_element) = last_element
                        .query_selector(".message-content")
                        .unwrap_or(None) 
                    {
                        // Обновляем только содержимое последнего сообщения
                        if let Some(last_message) = messages.last() {
                            // Для сообщений ассистента используем Markdown рендеринг
                            if last_message.role == "assistant" && !last_message.content.is_empty() {
                                let markdown_html = processMarkdown(&last_message.content);
                                content_element.set_inner_html(&markdown_html);
                            } else {
                                content_element.set_text_content(Some(&last_message.content));
                            }
                        }
                        
                        // Мягко прокручиваем вниз только если пользователь уже был внизу
                        let is_at_bottom = container.scroll_top() + container.client_height() >= container.scroll_height() - 50;
                        if is_at_bottom {
                            container.set_scroll_top(container.scroll_height());
                        }
                    }
                }
            }
        }
    }
}

pub fn set_typing_indicator(typing: bool) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let chat_container = document.get_element_by_id("chat-messages");
    
    if let Some(container) = chat_container {
        // Ищем все сообщения
        if let Ok(messages) = container.query_selector_all(".message") {
            let last_index = messages.length() - 1;
            
            if let Some(last_node) = messages.item(last_index) {
                // Преобразуем Node в Element и используем unchecked_into для HtmlElement
                if let Some(element) = last_node.dyn_ref::<web_sys::Element>() {
                    let html_element: &web_sys::HtmlElement = element.unchecked_ref();
                    let class_list = html_element.class_list();
                    if typing {
                        let _ = class_list.add_1("typing");
                    } else {
                        let _ = class_list.remove_1("typing");
                    }
                }
            }
        }
    }
}

pub fn show_loading() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    if let Some(loading) = document.get_element_by_id("loading-indicator") {
        loading.set_attribute("style", "display: block").unwrap();
    }
    if let Some(send_btn) = document.get_element_by_id("send-button") {
        send_btn.set_attribute("disabled", "true").unwrap();
    }
}

pub fn hide_loading() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    if let Some(loading) = document.get_element_by_id("loading-indicator") {
        loading.set_attribute("style", "display: none").unwrap();
    }
    if let Some(send_btn) = document.get_element_by_id("send-button") {
        send_btn.remove_attribute("disabled").unwrap();
    }
} 