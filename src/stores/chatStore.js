// Mock mode for frontend-only testing
const MOCK_MODE = true

let mockChats = [
  { id: '1', title: '欢迎使用 MyBudy', model_provider: 'kimi', model_name: 'moonshot-v1-8k', created_at: new Date().toISOString(), updated_at: new Date().toISOString() },
  { id: '2', title: '新对话', model_provider: 'openai', model_name: 'gpt-4', created_at: new Date().toISOString(), updated_at: new Date().toISOString() },
]

let mockMessages = {
  '1': [
    { id: '1', chat_id: '1', role: 'assistant', content: '你好！我是 MyBudy，你的 AI 助手。\n\n我可以帮你：\n- 💬 聊天对话\n- 📁 读取本地文件\n- 📸 截图识别\n- 🎤 语音输入\n\n点击左下角设置可以配置 AI 模型。', created_at: new Date().toISOString() },
  ],
  '2': [],
}

export async function getChats() {
  if (MOCK_MODE) {
    return mockChats
  }
  const { invoke } = await import('@tauri-apps/api/core')
  return await invoke('get_chats')
}

export async function createChat(title, modelProvider, modelName) {
  if (MOCK_MODE) {
    const chat = {
      id: Date.now().toString(),
      title: title || '新对话',
      model_provider: modelProvider || 'kimi',
      model_name: modelName || 'moonshot-v1-8k',
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    }
    mockChats.push(chat)
    mockMessages[chat.id] = []
    return chat
  }
  const { invoke } = await import('@tauri-apps/api/core')
  return await invoke('create_chat', { title, modelProvider, modelName })
}

export async function deleteChat(chatId) {
  if (MOCK_MODE) {
    mockChats = mockChats.filter(c => c.id !== chatId)
    delete mockMessages[chatId]
    return
  }
  const { invoke } = await import('@tauri-apps/api/core')
  return await invoke('delete_chat', { chatId })
}

export async function getMessages(chatId) {
  if (MOCK_MODE) {
    return mockMessages[chatId] || []
  }
  const { invoke } = await import('@tauri-apps/api/core')
  return await invoke('get_messages', { chatId })
}

export async function sendMessage(chatId, role, content, attachments = null) {
  if (MOCK_MODE) {
    const message = {
      id: Date.now().toString(),
      chat_id: chatId,
      role,
      content,
      attachments,
      created_at: new Date().toISOString(),
    }
    if (!mockMessages[chatId]) {
      mockMessages[chatId] = []
    }
    mockMessages[chatId].push(message)
    
    // Simulate AI response
    if (role === 'user') {
      setTimeout(() => {
        const response = {
          id: (Date.now() + 1).toString(),
          chat_id: chatId,
          role: 'assistant',
          content: `收到你的消息："${content}"\n\n（这是模拟回复，实际运行时会调用 AI API）`,
          created_at: new Date().toISOString(),
        }
        mockMessages[chatId].push(response)
      }, 500)
    }
    
    return message
  }
  const { invoke } = await import('@tauri-apps/api/core')
  return await invoke('send_message', { chatId, role, content, attachments })
}
