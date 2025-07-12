Write-Host "🦀 Rust WASM LLM Chat - Сборка и запуск" -ForegroundColor Green
Write-Host "======================================" -ForegroundColor Green

# Проверяем наличие wasm-pack
if (!(Get-Command wasm-pack -ErrorAction SilentlyContinue)) {
    Write-Host "❌ wasm-pack не найден!" -ForegroundColor Red
    Write-Host "Установите его командой: cargo install wasm-pack" -ForegroundColor Yellow
    exit 1
}

# Проверяем наличие Python
if (!(Get-Command python -ErrorAction SilentlyContinue) -and !(Get-Command python3 -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Python не найден!" -ForegroundColor Red
    Write-Host "Установите Python для запуска локального сервера" -ForegroundColor Yellow
    exit 1
}

# Собираем проект
Write-Host "🔨 Сборка WASM модуля..." -ForegroundColor Cyan
wasm-pack build --target web --out-dir pkg --out-name rust_llm_chat

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Сборка завершена успешно!" -ForegroundColor Green
    Write-Host "🚀 Запуск локального сервера..." -ForegroundColor Cyan
    
    # Запускаем Python сервер
    if (Get-Command python3 -ErrorAction SilentlyContinue) {
        python3 serve.py
    } else {
        python serve.py
    }
} else {
    Write-Host "❌ Ошибка при сборке!" -ForegroundColor Red
    exit 1
} 