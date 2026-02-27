import { getMessages, sendMessage as sendMessageApi } from '../stores/chatStore.js'
import { getModels } from '../stores/configStore.js'

export default class ChatArea {
  constructor({ onSendMessage }) {
    this.onSendMessage = onSendMessage
    this.chat = null
    this.messages = []
    this.models = []
    this.selectedModel = null
  }

  mount(container) {
    this.container = container
    this.container.innerHTML = `
      <div class="chat-area">
        <div class="chat-header">
          <div class="chat-title">
            <span class="title-text">Select a chat</span>
          </div>
          <div class="model-selector">
            <select class="model-select">
              <option>Loading models...</option>
            </select>
          </div>
        </div>
        
        <div class="messages-container">
          <div class="welcome-screen">
            <div class="welcome-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <circle cx="12" cy="12" r="10" />
                <path d="M8 14s1.5 2 4 2 4-2 4-2" />
                <line x1="9" y1="9" x2="9.01" y2="9" />
                <line x1="15" y1="9" x2="15.01" y2="9" />
              </svg>
            </div>
            <h2>Welcome to MyBudy</h2>
            <p>Your AI assistant with local capabilities</p>
            <div class="quick-actions">
              <button class="quick-btn" data-action="file">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
                  <polyline points="14 2 14 8 20 8" />
                </svg>
                Read file
              </button>
              <button class="quick-btn" data-action="screenshot">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
                  <circle cx="8.5" cy="8.5" r="1.5" />
                  <polyline points="21 15 16 10 5 21" />
                </svg>
                Screenshot
              </button>
              <button class="quick-btn" data-action="voice">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z" />
                  <path d="M19 10v2a7 7 0 0 1-14 0v-2" />
                  <line x1="12" y1="19" x2="12" y2="23" />
                  <line x1="8" y1="23" x2="16" y2="23" />
                </svg>
                Voice
              </button>
            </div>
          </div>
        </div>
        
        <div class="input-area">
          <div class="input-container">
            <button class="attach-btn" title="Attach file">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48" />
              </svg>
            </button>
            <textarea 
              class="message-input" 
              placeholder="Type a message..." 
              rows="1"
            ></textarea>
            <button class="send-btn" title="Send">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="22" y1="2" x2="11" y2="13" />
                <polygon points="22 2 15 22 11 13 2 9 22 2" />
              </svg>
            </button>
          </div>
          <div class="input-hint">Press Enter to send, Shift+Enter for new line</div>
        </div>
      </div>
    `

    this.bindEvents()
    this.loadModels()
  }

  bindEvents() {
    const input = this.container.querySelector('.message-input')
    const sendBtn = this.container.querySelector('.send-btn')
    const attachBtn = this.container.querySelector('.attach-btn')
    const modelSelect = this.container.querySelector('.model-select')

    input.addEventListener('keydown', (e) => {
      if (e.key === 'Enter' && !e.shiftKey) {
        e.preventDefault()
        this.handleSend()
      }
    })

    input.addEventListener('input', () => {
      input.style.height = 'auto'
      input.style.height = Math.min(input.scrollHeight, 200) + 'px'
    })

    sendBtn.addEventListener('click', () => this.handleSend())
    attachBtn.addEventListener('click', () => this.handleAttach())
    modelSelect.addEventListener('change', (e) => {
      this.selectedModel = e.target.value
    })

    // Quick actions
    this.container.querySelectorAll('.quick-btn').forEach(btn => {
      btn.addEventListener('click', () => {
        const action = btn.dataset.action
        this.handleQuickAction(action)
      })
    })
  }

  async loadModels() {
    try {
      const models = await getModels()
      this.models = models.providers.filter(p => p.enabled)
      
      const select = this.container.querySelector('.model-select')
      select.innerHTML = this.models.map(provider => 
        provider.models.map(model => 
          `<option value="${provider.id}:${model}">${provider.name} - ${model}</option>`
        ).join('')
      ).join('')
      
      if (this.models.length > 0) {
        this.selectedModel = `${this.models[0].id}:${this.models[0].default_model}`
        select.value = this.selectedModel
      }
    } catch (e) {
      console.error('Failed to load models:', e)
    }
  }

  setChat(chat) {
    this.chat = chat
    this.container.querySelector('.title-text').textContent = chat.title
    this.container.querySelector('.welcome-screen')?.remove()
    this.loadMessages()
  }

  clear() {
    this.chat = null
    this.messages = []
    this.container.querySelector('.title-text').textContent = 'Select a chat'
    this.container.querySelector('.messages-container').innerHTML = ''
  }

  async loadMessages() {
    if (!this.chat) return
    
    try {
      this.messages = await getMessages(this.chat.id)
      this.renderMessages()
    } catch (e) {
      console.error('Failed to load messages:', e)
    }
  }

  renderMessages() {
    const container = this.container.querySelector('.messages-container')
    
    if (this.messages.length === 0) {
      container.innerHTML = `
        <div class="empty-messages">
          <p>No messages yet. Start a conversation!</p>
        </div>
      `
      return
    }

    container.innerHTML = this.messages.map(msg => this.renderMessage(msg)).join('')
    this.scrollToBottom()
  }

  renderMessage(msg) {
    const isUser = msg.role === 'user'
    return `
      <div class="message ${isUser ? 'user' : 'assistant'}">
        <div class="message-avatar">
          ${isUser 
            ? `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" />
                <circle cx="12" cy="7" r="4" />
              </svg>`
            : `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10" />
                <path d="M8 14s1.5 2 4 2 4-2 4-2" />
                <line x1="9" y1="9" x2="9.01" y2="9" />
                <line x1="15" y1="9" x2="15.01" y2="9" />
              </svg>`
          }
        </div>
        <div class="message-content">
          <div class="message-text">${this.escapeHtml(msg.content)}</div>
          <div class="message-time">${this.formatTime(msg.created_at)}</div>
        </div>
      </div>
    `
  }

  async sendMessage(content, chatId) {
    try {
      // Add user message
      await sendMessageApi(chatId, 'user', content)
      await this.loadMessages()
      
      // TODO: Call AI API and add assistant response
      // For now, just echo back
      await sendMessageApi(chatId, 'assistant', `Echo: ${content}`)
      await this.loadMessages()
    } catch (e) {
      console.error('Failed to send message:', e)
    }
  }

  handleSend() {
    const input = this.container.querySelector('.message-input')
    const content = input.value.trim()
    
    if (!content) return
    
    input.value = ''
    input.style.height = 'auto'
    
    if (this.chat) {
      this.sendMessage(content, this.chat.id)
    } else {
      this.onSendMessage(content)
    }
  }

  handleAttach() {
    // TODO: Implement file attachment
    alert('File attachment coming soon!')
  }

  handleQuickAction(action) {
    switch (action) {
      case 'file':
        this.handleAttach()
        break
      case 'screenshot':
        this.takeScreenshot()
        break
      case 'voice':
        this.startVoice()
        break
    }
  }

  async takeScreenshot() {
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      const screenshot = await invoke('capture_screen')
      console.log('Screenshot:', screenshot)
      // TODO: Handle screenshot
    } catch (e) {
      console.error('Failed to capture screen:', e)
    }
  }

  async startVoice() {
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('start_voice_recognition')
    } catch (e) {
      console.error('Failed to start voice:', e)
    }
  }

  scrollToBottom() {
    const container = this.container.querySelector('.messages-container')
    container.scrollTop = container.scrollHeight
  }

  escapeHtml(text) {
    const div = document.createElement('div')
    div.textContent = text
    return div.innerHTML
  }

  formatTime(dateString) {
    const date = new Date(dateString)
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
  }
}
