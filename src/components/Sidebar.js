import { getChats, createChat, deleteChat } from '../stores/chatStore.js'

export default class Sidebar {
  constructor({ onChatSelect, onNewChat, onDeleteChat }) {
    this.onChatSelect = onChatSelect
    this.onNewChat = onNewChat
    this.onDeleteChat = onDeleteChat
    this.chats = []
    this.activeChatId = null
  }

  mount(container) {
    this.container = container
    this.container.innerHTML = `
      <div class="sidebar">
        <div class="sidebar-header">
          <div class="logo">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="10" />
              <path d="M8 14s1.5 2 4 2 4-2 4-2" />
              <line x1="9" y1="9" x2="9.01" y2="9" />
              <line x1="15" y1="9" x2="15.01" y2="9" />
            </svg>
            <span>MyBudy</span>
          </div>
          <button class="new-chat-btn" title="New Chat">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="12" y1="5" x2="12" y2="19" />
              <line x1="5" y1="12" x2="19" y2="12" />
            </svg>
          </button>
        </div>
        
        <div class="sidebar-search">
          <input type="text" placeholder="Search chats..." class="search-input" />
        </div>
        
        <div class="chat-list"></div>
        
        <div class="sidebar-footer">
          <button class="settings-btn">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="3" />
              <path d="M12 1v6m0 6v6m4.22-10.22l4.24-4.24M6.34 6.34L2.1 2.1m17.8 17.8l-4.24-4.24M6.34 17.66l-4.24 4.24M23 12h-6m-6 0H1m20.24-4.24l-4.24 4.24M6.34 6.34l-4.24-4.24" />
            </svg>
            <span>Settings</span>
          </button>
        </div>
      </div>
    `

    this.bindEvents()
  }

  bindEvents() {
    const newChatBtn = this.container.querySelector('.new-chat-btn')
    newChatBtn.addEventListener('click', () => this.onNewChat())

    const searchInput = this.container.querySelector('.search-input')
    searchInput.addEventListener('input', (e) => this.filterChats(e.target.value))

    const settingsBtn = this.container.querySelector('.settings-btn')
    settingsBtn.addEventListener('click', () => this.showSettings())
  }

  setChats(chats) {
    this.chats = chats
    this.renderChatList()
  }

  setActiveChat(chatId) {
    this.activeChatId = chatId
    this.renderChatList()
  }

  renderChatList() {
    const chatList = this.container.querySelector('.chat-list')
    
    if (this.chats.length === 0) {
      chatList.innerHTML = `
        <div class="empty-chats">
          <p>No chats yet</p>
          <p class="hint">Click + to start a new chat</p>
        </div>
      `
      return
    }

    chatList.innerHTML = this.chats.map(chat => `
      <div class="chat-item ${chat.id === this.activeChatId ? 'active' : ''}" data-id="${chat.id}">
        <div class="chat-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
          </svg>
        </div>
        <div class="chat-info">
          <div class="chat-title">${this.escapeHtml(chat.title)}</div>
          <div class="chat-meta">${this.formatDate(chat.updated_at)}</div>
        </div>
        <button class="delete-btn" title="Delete">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="3 6 5 6 21 6" />
            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
          </svg>
        </button>
      </div>
    `).join('')

    // Bind events
    chatList.querySelectorAll('.chat-item').forEach(item => {
      item.addEventListener('click', (e) => {
        if (!e.target.closest('.delete-btn')) {
          const chat = this.chats.find(c => c.id === item.dataset.id)
          if (chat) this.onChatSelect(chat)
        }
      })
    })

    chatList.querySelectorAll('.delete-btn').forEach(btn => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation()
        const item = btn.closest('.chat-item')
        const chat = this.chats.find(c => c.id === item.dataset.id)
        if (chat && confirm('Delete this chat?')) {
          this.onDeleteChat(chat)
        }
      })
    })
  }

  filterChats(query) {
    const items = this.container.querySelectorAll('.chat-item')
    const lowerQuery = query.toLowerCase()
    
    items.forEach(item => {
      const title = item.querySelector('.chat-title').textContent.toLowerCase()
      item.style.display = title.includes(lowerQuery) ? 'flex' : 'none'
    })
  }

  showSettings() {
    // TODO: Implement settings modal
    alert('Settings coming soon!')
  }

  escapeHtml(text) {
    const div = document.createElement('div')
    div.textContent = text
    return div.innerHTML
  }

  formatDate(dateString) {
    const date = new Date(dateString)
    const now = new Date()
    const diff = now - date
    
    if (diff < 60000) return 'Just now'
    if (diff < 3600000) return `${Math.floor(diff / 60000)}m ago`
    if (diff < 86400000) return `${Math.floor(diff / 3600000)}h ago`
    if (diff < 604800000) return `${Math.floor(diff / 86400000)}d ago`
    
    return date.toLocaleDateString()
  }
}
