// Mock mode for frontend-only testing
const MOCK_MODE = true

const mockConfig = {
  theme: 'dark',
  language: 'zh-CN',
  auto_start: false,
  voice_enabled: true,
  default_model: 'kimi',
  shortcuts: {
    show_window: 'CmdOrCtrl+Shift+M',
    voice_wake: 'CmdOrCtrl+Shift+V',
    screenshot: 'CmdOrCtrl+Shift+S',
  }
}

const mockModels = {
  providers: [
    {
      id: 'kimi',
      name: 'Kimi',
      provider: 'kimi',
      api_key: '',
      api_url: 'https://api.moonshot.cn/v1',
      models: ['moonshot-v1-8k', 'moonshot-v1-32k', 'moonshot-v1-128k'],
      default_model: 'moonshot-v1-8k',
      enabled: true,
    },
    {
      id: 'openai',
      name: 'OpenAI',
      provider: 'openai',
      api_key: '',
      api_url: 'https://api.openai.com/v1',
      models: ['gpt-4', 'gpt-4-turbo', 'gpt-3.5-turbo'],
      default_model: 'gpt-4',
      enabled: false,
    },
  ]
}

export async function getConfig() {
  if (MOCK_MODE) {
    return mockConfig
  }
  const { invoke } = await import('@tauri-apps/api/core')
  return await invoke('get_config')
}

export async function setConfig(config) {
  if (MOCK_MODE) {
    Object.assign(mockConfig, config)
    return
  }
  const { invoke } = await import('@tauri-apps/api/core')
  return await invoke('set_config', { config })
}

export async function getModels() {
  if (MOCK_MODE) {
    return mockModels
  }
  const { invoke } = await import('@tauri-apps/api/core')
  return await invoke('get_models')
}

export async function setModels(models) {
  if (MOCK_MODE) {
    Object.assign(mockModels, models)
    return
  }
  const { invoke } = await import('@tauri-apps/api/core')
  return await invoke('set_models', { models })
}
