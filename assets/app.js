/**
 * Rust WASM LLM Chat Application
 * Main JavaScript module for chat functionality
 */

// Global variables
let chatApp = null;

/**
 * Process Markdown text to HTML
 * @param {string} text - Raw markdown text
 * @returns {string} - HTML formatted text
 */
function processMarkdown(text) {
    // Configure marked for safe rendering
    if (typeof marked !== 'undefined') {
        marked.setOptions({
            breaks: true,
            gfm: true,
            sanitize: false,
            smartLists: true,
            smartypants: true
        });
        return marked.parse(text);
    }
    // Fallback if marked is not loaded
    return text.replace(/\n/g, '<br>');
}

/**
 * Initialize the chat application
 */
async function initializeChatApp() {
    try {
        // Initialize WASM module
        await init();
        
        // Create chat app instance
        chatApp = new ChatApp();
        
        // Set initialization time
        const initTimeElement = document.getElementById('init-time');
        if (initTimeElement) {
            initTimeElement.textContent = new Date().toLocaleTimeString('ru-RU');
        }
        
        // Setup event listeners
        setupEventListeners();
        
        console.log('Chat app initialized successfully');
    } catch (error) {
        console.error('Failed to initialize chat app:', error);
        showError('Не удалось инициализировать чат: ' + error.message);
    }
}

/**
 * Setup all event listeners for the chat interface
 */
function setupEventListeners() {
    // Get DOM elements
    const messageInput = document.getElementById('message-input');
    const sendButton = document.getElementById('send-button');
    const clearButton = document.getElementById('clear-button');
    const modelSelect = document.getElementById('model-select');

    // Send message button handler
    if (sendButton) {
        sendButton.addEventListener('click', sendMessage);
    }
    
    // Enter key handler (with Shift+Enter support for new lines)
    if (messageInput) {
        messageInput.addEventListener('keydown', (e) => {
            if (e.key === 'Enter' && !e.shiftKey) {
                e.preventDefault();
                sendMessage();
            }
        });

        // Auto-resize textarea
        messageInput.addEventListener('input', () => {
            messageInput.style.height = 'auto';
            messageInput.style.height = Math.min(messageInput.scrollHeight, 120) + 'px';
        });
    }

    // Clear chat button handler
    if (clearButton) {
        clearButton.addEventListener('click', () => {
            if (confirm('Вы уверены, что хотите очистить историю чата?')) {
                chatApp.clear_chat();
            }
        });
    }

    // Model selection handler
    if (modelSelect) {
        modelSelect.addEventListener('change', (e) => {
            const selectedModel = e.target.value;
            if (chatApp) {
                chatApp.set_model(selectedModel);
                console.log('Model changed to:', selectedModel);
            }
        });
    }
}

/**
 * Send a message to the chat
 */
async function sendMessage() {
    const messageInput = document.getElementById('message-input');
    if (!messageInput || !chatApp) {
        return;
    }

    const message = messageInput.value.trim();
    if (!message) {
        return;
    }

    try {
        // Clear input field
        messageInput.value = '';
        messageInput.style.height = 'auto';
        
        // Send message to chat app
        await chatApp.send_message(message);
    } catch (error) {
        console.error('Error sending message:', error);
        showError('Ошибка при отправке сообщения: ' + error.message);
    }
}

/**
 * Display error message in chat
 * @param {string} message - Error message to display
 */
function showError(message) {
    const chatMessages = document.getElementById('chat-messages');
    if (!chatMessages) {
        return;
    }

    const errorDiv = document.createElement('div');
    errorDiv.className = 'error-message';
    errorDiv.textContent = message;
    chatMessages.appendChild(errorDiv);
    chatMessages.scrollTop = chatMessages.scrollHeight;
}

/**
 * Main application entry point
 */
async function main() {
    // Make processMarkdown available globally for Rust
    window.processMarkdown = processMarkdown;
    
    // Initialize the chat application
    await initializeChatApp();
}

// Start the application when DOM is loaded
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', main);
} else {
    main();
} 