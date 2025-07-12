use crate::types::ChatMessage;
use wasm_bindgen::prelude::*;
use gloo_net::http::Request;

pub async fn call_llm_api(messages: &[ChatMessage], model: &str) -> Result<String, JsValue> {
    // Создаем запрос для локального Ollama API
    let mut api_messages = Vec::new();
    
    // Добавляем системное сообщение
    api_messages.push(ChatMessage {
        role: "system".to_string(),
        content: "Speak only in Russian, говори ТОЛЬКО НА РУССКОМ ЯЗЫКЕ! Ты - помощник по всем вопросам, умный и точный.".to_string(),
        timestamp: 0.0,
    });
    
    // Добавляем историю сообщений
    for msg in messages {
        if msg.role != "system" {
            api_messages.push(ChatMessage {
                role: msg.role.clone(),
                content: msg.content.clone(),
                timestamp: 0.0,
            });
        }
    }
    
    // Создаем запрос в формате Ollama без потоковой передачи
    let request_body = serde_json::json!({
        "model": model,
        "messages": api_messages.iter().map(|msg| {
            serde_json::json!({
                "role": msg.role,
                "content": msg.content
            })
        }).collect::<Vec<_>>(),
        "stream": false
    });

    let response = Request::post("http://localhost:3001/api/chat")
        .header("Content-Type", "application/json")
        .body(request_body.to_string())
        .map_err(|e| JsValue::from_str(&format!("Request error: {:?}", e)))?
        .send()
        .await
        .map_err(|e| JsValue::from_str(&format!("Send error: {:?}", e)))?;

    if response.ok() {
        let response_text = response
            .text()
            .await
            .map_err(|e| JsValue::from_str(&format!("Response text error: {:?}", e)))?;
        
        // Парсим ответ от Ollama API
        let parsed: serde_json::Value = serde_json::from_str(&response_text)
            .map_err(|e| JsValue::from_str(&format!("JSON parse error: {:?}", e)))?;
        
        // Ollama возвращает ответ в поле message.content
        if let Some(content) = parsed["message"]["content"].as_str() {
            Ok(content.to_string())
        } else {
            Err(JsValue::from_str("Invalid response format"))
        }
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(JsValue::from_str(&format!("API request failed: {}", error_text)))
    }
} 