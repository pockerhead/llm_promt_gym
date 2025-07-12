# 🤖 Rust WASM LLM Chat

Интерактивный чат с искусственным интеллектом, написанный на Rust с использованием WebAssembly. Поддерживает полноценное отображение Markdown и современный пользовательский интерфейс.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=webassembly&logoColor=white)
![JavaScript](https://img.shields.io/badge/JavaScript-323330?style=for-the-badge&logo=javascript&logoColor=F7DF1E)

## ✨ Особенности

- 🚀 **Высокая производительность**: Rust + WebAssembly обеспечивают быструю работу
- 📝 **Полная поддержка Markdown**: Красивое отображение кода, таблиц, списков и форматирования
- ⚡ **Типинг-анимация**: Реалистичная анимация печати как в ChatGPT
- 🎨 **Современный UI**: Градиентный дизайн с плавными анимациями
- 🔧 **Модульная архитектура**: Четкое разделение логики на отдельные модули
- 🌐 **Поддержка множества моделей**: llama3-8b/70b-instruct, codellama, mistral, gemma, qwen2
- 🔗 **Интеграция с Ollama**: Локальный запуск LLM моделей
- 🇷🇺 **Русская локализация**: Полная поддержка русского языка

## 🛠️ Технологический стек

- **Backend**: Rust, WebAssembly, wasm-bindgen
- **Frontend**: HTML5, CSS3, JavaScript (ES6+)
- **Markdown**: marked.js для рендеринга
- **API**: Ollama REST API
- **Сборка**: wasm-pack
- **Стили**: Современный CSS Grid/Flexbox

## 📁 Структура проекта

```
src/
├── lib.rs          # Главный модуль и точка входа
├── types.rs        # Структуры данных (ChatMessage, ChatRequest, etc.)
├── api.rs          # Интеграция с Ollama API
├── ui.rs           # Обновления пользовательского интерфейса
├── chat.rs         # Основная логика чата
└── bindings.rs     # JavaScript биндинги
```

## 🚀 Установка и запуск

### Предварительные требования

1. **Rust и Cargo**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **wasm-pack**
   ```bash
   cargo install wasm-pack
   ```

3. **Ollama** (для LLM API)
   ```bash
   curl -fsSL https://ollama.ai/install.sh | sh
   ```

### Сборка проекта

1. **Клонирование репозитория**
   ```bash
   git clone https://github.com/pockerhead/llm_promt_gym.git
   cd llm_promt_gym
   ```

2. **Сборка WASM модуля**
   ```bash
   wasm-pack build --target web --out-dir pkg
   ```

3. **Запуск локального сервера**
   ```bash
   # Python 3
   python -m http.server 8000
   
   # или Python 2
   python -m SimpleHTTPServer 8000
   ```

4. **Настройка Ollama**
   ```bash
   # Запуск Ollama сервера
   ollama serve
   
   # Загрузка модели (в другом терминале)
   ollama pull llama3-8b-instruct
   ```

5. **Открытие в браузере**
   ```
   http://localhost:8000
   ```

## 🎯 Использование

1. **Выбор модели**: Используйте выпадающий список для выбора LLM модели
2. **Отправка сообщений**: Введите текст и нажмите Enter или кнопку "Отправить"
3. **Markdown поддержка**: Все ответы ассистента поддерживают полный Markdown
4. **Очистка чата**: Используйте кнопку "Очистить" для очистки истории

## 🔧 Конфигурация

Для изменения API endpoint отредактируйте файл `src/api.rs`:

```rust
let response = Request::post("http://localhost:3001/api/chat")
    .header("Content-Type", "application/json")
    .body(request_body.to_string())
    // ...
```

## 📝 Поддерживаемые модели

- **llama3-8b-instruct** - Быстрая и качественная модель
- **llama3-70b-instruct** - Более мощная модель (требует больше ресурсов)
- **codellama** - Специализированная модель для кода
- **mistral** - Эффективная модель от Mistral AI
- **gemma** - Модель от Google
- **qwen2** - Модель от Alibaba

## 🎨 Markdown возможности

Поддерживаются все основные элементы Markdown:

- **Заголовки** (H1-H6)
- **Форматирование текста** (жирный, курсив, зачеркнутый)
- **Списки** (маркированные и нумерованные)
- **Код** (inline и блоки кода)
- **Таблицы**
- **Цитаты**
- **Ссылки**
- **Разделители**

## 🤝 Вклад в проект

Приветствуются любые улучшения! Пожалуйста:

1. Создайте форк репозитория
2. Создайте ветку для вашей функции (`git checkout -b feature/amazing-feature`)
3. Зафиксируйте изменения (`git commit -m 'Add amazing feature'`)
4. Отправьте в ветку (`git push origin feature/amazing-feature`)
5. Создайте Pull Request

## 📄 Лицензия

Этот проект распространяется под лицензией MIT. Подробности в файле `LICENSE`.

## 🔗 Полезные ссылки

- [Документация Rust](https://doc.rust-lang.org/)
- [WebAssembly](https://webassembly.org/)
- [Ollama](https://ollama.ai/)
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)

## 🆘 Поддержка

Если у вас есть вопросы или проблемы, создайте issue в репозитории.

---

⭐ Если проект был полезен, поставьте звездочку! 