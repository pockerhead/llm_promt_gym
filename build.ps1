# Сборка WASM модуля с помощью wasm-pack
Write-Host "🦀 Сборка Rust WASM модуля..." -ForegroundColor Green
wasm-pack build --target web --out-dir pkg --out-name rust_llm_chat

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Сборка завершена!" -ForegroundColor Green
    Write-Host "📂 WASM файлы находятся в папке pkg/" -ForegroundColor Yellow
    Write-Host "🌐 Откройте index.html в браузере для запуска приложения" -ForegroundColor Cyan
    Write-Host "🚀 Убедитесь, что ваш Ollama API доступен по адресу http://localhost:3001/api/chat" -ForegroundColor Magenta
} else {
    Write-Host "❌ Ошибка при сборке!" -ForegroundColor Red
} 