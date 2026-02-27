import Sidebar from './Sidebar.js'
import ChatArea from './ChatArea.js'
import WelcomeScreen from './WelcomeScreen.js'
import { getChats, createChat, deleteChat } from '../stores/chatStore.js'
import { getConfig } from '../stores/configStore.js'

export default class App {
  constructor() {
    this.currentChat = null
    this.chats = []
    this.config = null
    this.showWelcome = true
  }

  async mount(container) {
    this.container = container
    
    // Check if first time
    const hasSeenWelcome = localStorage.getItem('mybudy-welcome-seen')
    if (!hasSeenWelcome) {
      this.showWelcomeScreen()
    } else {
      this.showMainApp()
    }
  }

  showWelcomeScreen() {
    this.welcomeScreen = new WelcomeScreen({
      onStart: () => {
        localStorage.setItem('mybudy-welcome-seen', 'true')
        this.welcomeScreen.unmount()
        this.showMainApp()
      }
    })
    this.welcomeScreen.mount(this.container)
  }

  showMainApp() {
    this.container.innerHTML = `
      <div class="app">
        <div class="sidebar-container"></div>
        <div class="chat-container"></div>
      </div>
    `

    // Initialize components
    this.sidebar = new Sidebar({
      onChatSelect: (chat) => this.selectChat(chat),
      onNewChat: () => this.createNewChat(),
      onDeleteChat: (chat) => this.deleteChat(chat),
    })

    this.chatArea = new ChatArea({
      onSendMessage: (content) => this.sendMessage(content),
    })

    this.sidebar.mount(this.container.querySelector('.sidebar-container'))
    this.chatArea.mount(this.container.querySelector('.chat-container'))

    // Load chats
    this.loadChats()
  }

  async loadChats() {
    this.chats = await getChats()
    this.sidebar.setChats(this.chats)
    
    if (this.chats.length > 0 && !this.currentChat) {
      this.selectChat(this.chats[0])
    }
  }

  selectChat(chat) {
    this.currentChat = chat
    this.sidebar.setActiveChat(chat.id)
    this.chatArea.setChat(chat)
  }

  async createNewChat() {
    const chat = await createChat('New Chat', 'kimi', 'moonshot-v1-8k')
    await this.loadChats()
    this.selectChat(chat)
  }

  async deleteChat(chat) {
    await deleteChat(chat.id)
    if (this.currentChat?.id === chat.id) {
      this.currentChat = null
      this.chatArea.clear()
    }
    await this.loadChats()
  }

  async sendMessage(content) {
    if (!this.currentChat) {
      await this.createNewChat()
    }
    
    await this.chatArea.sendMessage(content, this.currentChat.id)
  }
}
