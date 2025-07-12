#!/bin/bash

# Сборка WASM модуля с помощью wasm-pack
echo "🦀 Сборка Rust WASM модуля..."
wasm-pack build --target web --out-dir pkg --out-name rust_llm_chat

echo "✅ Сборка завершена!"
echo "📂 WASM файлы находятся в папке pkg/"
echo "🌐 Откройте index.html в браузере для запуска приложения"
echo "🚀 Убедитесь, что ваш Ollama API доступен по адресу http://localhost:3001/api/chat" 