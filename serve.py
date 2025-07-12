#!/usr/bin/env python3
"""
Простой HTTP сервер для тестирования Rust WASM LLM Chat
"""
import http.server
import socketserver
import webbrowser
import os
import sys

PORT = 8000

class CORSHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        super().end_headers()

def main():
    try:
        # Проверяем, есть ли файл index.html
        if not os.path.exists('index.html'):
            print("❌ Файл index.html не найден!")
            print("Убедитесь, что вы запускаете сервер из корневой папки проекта")
            sys.exit(1)
        
        # Проверяем, есть ли папка pkg (собранные WASM файлы)
        if not os.path.exists('pkg'):
            print("⚠️  Папка pkg не найдена!")
            print("Сначала соберите проект:")
            print("  Windows: .\\build.ps1")
            print("  Linux/macOS: ./build.sh")
            print("  Или вручную: wasm-pack build --target web --out-dir pkg --out-name rust_llm_chat")
            sys.exit(1)
            
        # Запускаем сервер
        with socketserver.TCPServer(("", PORT), CORSHTTPRequestHandler) as httpd:
            print(f"🚀 Сервер запущен на http://localhost:{PORT}")
            print("📂 Обслуживаем файлы из текущей папки")
            print("🛑 Нажмите Ctrl+C для остановки")
            
            # Открываем браузер
            webbrowser.open(f'http://localhost:{PORT}')
            
            # Запускаем сервер
            httpd.serve_forever()
            
    except KeyboardInterrupt:
        print("\n👋 Сервер остановлен")
    except Exception as e:
        print(f"❌ Ошибка при запуске сервера: {e}")

if __name__ == "__main__":
    main() 