export default class WelcomeScreen {
  constructor({ onStart }) {
    this.onStart = onStart
  }

  mount(container) {
    this.container = container
    this.container.innerHTML = `
      <div class="welcome-screen">
        <div class="welcome-content">
          <div class="logo-container">
            <img src="/src/assets/logo.png" alt="MyBudy" class="welcome-logo">
          </div>
          <h1 class="welcome-title">MyBudy</h1>
          <p class="welcome-subtitle">你的 AI 伙伴</p>
          <div class="welcome-features">
            <div class="feature-item">
              <span class="feature-icon">💬</span>
              <span class="feature-text">智能对话</span>
            </div>
            <div class="feature-item">
              <span class="feature-icon">📁</span>
              <span class="feature-text">本地文件</span>
            </div>
            <div class="feature-item">
              <span class="feature-icon">📸</span>
              <span class="feature-text">截图识别</span>
            </div>
            <div class="feature-item">
              <span class="feature-icon">🎤</span>
              <span class="feature-text">语音输入</span>
            </div>
          </div>
          <button class="welcome-button" id="start-btn">
            开始使用
          </button>
        </div>
      </div>
    `

    // 绑定事件
    const startBtn = this.container.querySelector('#start-btn')
    startBtn.addEventListener('click', () => {
      if (this.onStart) {
        this.onStart()
      }
    })
  }

  unmount() {
    this.container.innerHTML = ''
  }
}
