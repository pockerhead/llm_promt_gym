use crate::types::ChatMessage;
use crate::api::call_llm_api;
use crate::ui::{update_ui, update_last_message_only, set_typing_indicator, show_loading, hide_loading};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
pub struct ChatApp {
    messages: Vec<ChatMessage>,
    is_loading: bool,
    current_model: String,
    current_assistant_message: String,
}

#[wasm_bindgen]
impl ChatApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ChatApp {
        crate::console_log!("Initializing ChatApp");
        ChatApp {
            messages: Vec::new(),
            is_loading: false,
            current_model: "llama3-8b-instruct".to_string(),
            current_assistant_message: String::new(),
        }
    }

    #[wasm_bindgen]
    pub fn set_model(&mut self, model: &str) {
        self.current_model = model.to_string();
        crate::console_log!("Model set to: {}", model);
    }

    #[wasm_bindgen]
    pub fn add_message(&mut self, role: &str, content: &str) {
        let timestamp = js_sys::Date::now();
        let message = ChatMessage {
            role: role.to_string(),
            content: content.to_string(),
            timestamp,
        };
        self.messages.push(message);
        update_ui(&self.messages);
    }

    #[wasm_bindgen]
    pub fn start_assistant_message(&mut self) {
        self.current_assistant_message = String::new();
        let timestamp = js_sys::Date::now();
        let message = ChatMessage {
            role: "assistant".to_string(),
            content: "".to_string(),
            timestamp,
        };
        self.messages.push(message);
        update_ui(&self.messages);
        set_typing_indicator(true);
    }

    #[wasm_bindgen]
    pub fn append_to_assistant_message(&mut self, text: &str) {
        self.current_assistant_message.push_str(text);
        if let Some(last_message) = self.messages.last_mut() {
            if last_message.role == "assistant" {
                last_message.content = self.current_assistant_message.clone();
            }
        }
        update_last_message_only(&self.messages);
    }

    #[wasm_bindgen]
    pub fn finish_assistant_message(&mut self) {
        // Сообщение уже добавлено, просто очищаем буфер
        self.current_assistant_message.clear();
        set_typing_indicator(false);
    }

    #[wasm_bindgen]
    pub async fn send_message(&mut self, message: &str) -> Result<(), JsValue> {
        if self.is_loading {
            return Ok(());
        }

        self.is_loading = true;
        self.add_message("user", message);
        show_loading();

        let result = call_llm_api(&self.messages, &self.current_model).await;
        self.is_loading = false;
        hide_loading();

        match result {
            Ok(response) => {
                if let Err(e) = self.type_message_gradually(&response).await {
                    crate::console_log!("Error typing message: {:?}", e);
                    self.add_message("system", "Произошла ошибка при отображении сообщения");
                }
            }
            Err(e) => {
                crate::console_log!("Error calling LLM API: {:?}", e);
                self.add_message("system", "Произошла ошибка при обращении к LLM API");
            }
        }

        Ok(())
    }

    async fn type_message_gradually(&mut self, message: &str) -> Result<(), JsValue> {
        // Начинаем новое сообщение ассистента
        self.start_assistant_message();
        
        // Разбиваем сообщение на токены, сохраняя переносы строк
        let mut tokens = Vec::new();
        let mut current_word = String::new();
        
        for ch in message.chars() {
            if ch.is_whitespace() {
                if !current_word.is_empty() {
                    tokens.push(current_word.clone());
                    current_word.clear();
                }
                // Добавляем пробельный символ как отдельный токен
                tokens.push(ch.to_string());
            } else {
                current_word.push(ch);
            }
        }
        
        // Добавляем последнее слово, если оно есть
        if !current_word.is_empty() {
            tokens.push(current_word);
        }
        
        for token in tokens.iter() {
            self.append_to_assistant_message(token);
            
            // Более короткая задержка для пробелов и переносов строк
            let delay = if token.chars().all(|c| c.is_whitespace()) {
                10 // Быстрая задержка для пробельных символов
            } else {
                30 + (js_sys::Math::random() * 40.0) as u32 // Обычная задержка для слов
            };
            
            // Используем setTimeout для задержки
            let promise = js_sys::Promise::new(&mut |resolve, _| {
                let window = web_sys::window().unwrap();
                window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    &resolve,
                    delay as i32,
                ).unwrap();
            });
            
            JsFuture::from(promise).await?;
        }
        
        self.finish_assistant_message();
        Ok(())
    }

    #[wasm_bindgen]
    pub fn clear_chat(&mut self) {
        self.messages.clear();
        update_ui(&self.messages);
    }
} 