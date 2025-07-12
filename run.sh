#!/bin/bash

echo "🦀 Rust WASM LLM Chat - Сборка и запуск"
echo "======================================"

# Проверяем наличие wasm-pack
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack не найден!"
    echo "Установите его командой: cargo install wasm-pack"
    exit 1
fi

# Проверяем наличие Python
if ! command -v python3 &> /dev/null && ! command -v python &> /dev/null; then
    echo "❌ Python не найден!"
    echo "Установите Python для запуска локального сервера"
    exit 1
fi

# Собираем проект
echo "🔨 Сборка WASM модуля..."
wasm-pack build --target web --out-dir pkg --out-name rust_llm_chat

if [ $? -eq 0 ]; then
    echo "✅ Сборка завершена успешно!"
    echo "🚀 Запуск локального сервера..."
    
    # Запускаем Python сервер
    if command -v python3 &> /dev/null; then
        python3 serve.py
    else
        python serve.py
    fi
else
    echo "❌ Ошибка при сборке!"
    exit 1
fi 