import App from './components/App.js'

// Initialize app
document.addEventListener('DOMContentLoaded', () => {
  const app = new App()
  app.mount(document.getElementById('app'))
})
