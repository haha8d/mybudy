import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'

// Float button state
let isDragging = false
let dragStartX = 0
let dragStartY = 0
let buttonStartX = 0
let buttonStartY = 0

const floatButton = document.getElementById('float-button')
const notification = document.getElementById('notification')

// Make button draggable
floatButton.addEventListener('mousedown', startDrag)
floatButton.addEventListener('touchstart', startDrag, { passive: false })

document.addEventListener('mousemove', onDrag)
document.addEventListener('touchmove', onDrag, { passive: false })

document.addEventListener('mouseup', endDrag)
document.addEventListener('touchend', endDrag)

function startDrag(e) {
  isDragging = true
  const clientX = e.touches ? e.touches[0].clientX : e.clientX
  const clientY = e.touches ? e.touches[0].clientY : e.clientY
  
  dragStartX = clientX
  dragStartY = clientY
  
  const rect = floatButton.getBoundingClientRect()
  buttonStartX = rect.left
  buttonStartY = rect.top
  
  floatButton.classList.add('dragging')
}

function onDrag(e) {
  if (!isDragging) return
  e.preventDefault()
  
  const clientX = e.touches ? e.touches[0].clientX : e.clientX
  const clientY = e.touches ? e.touches[0].clientY : e.clientY
  
  const deltaX = clientX - dragStartX
  const deltaY = clientY - dragStartY
  
  floatButton.style.left = `${buttonStartX + deltaX}px`
  floatButton.style.top = `${buttonStartY + deltaY}px`
  floatButton.style.right = 'auto'
  floatButton.style.bottom = 'auto'
}

function endDrag(e) {
  if (!isDragging) return
  isDragging = false
  floatButton.classList.remove('dragging')
}

// Click to open main window
floatButton.addEventListener('click', (e) => {
  if (!isDragging) {
    invoke('show_main_window')
  }
})

// Drag and drop support
floatButton.addEventListener('dragover', (e) => {
  e.preventDefault()
  floatButton.classList.add('drag-over')
})

floatButton.addEventListener('dragleave', () => {
  floatButton.classList.remove('drag-over')
})

floatButton.addEventListener('drop', async (e) => {
  e.preventDefault()
  floatButton.classList.remove('drag-over')
  
  const files = e.dataTransfer.files
  if (files.length > 0) {
    // Show notification
    showNotification(`${files.length} file(s) dropped`)
    
    // Open main window with files
    invoke('show_main_window')
    
    // Store files for main window to process
    window.droppedFiles = Array.from(files)
  }
})

function showNotification(text) {
  notification.textContent = text
  notification.classList.add('show')
  
  setTimeout(() => {
    notification.classList.remove('show')
  }, 3000)
}

// Pulse animation on new message
window.showNotification = showNotification

// Listen for messages from main window
window.addEventListener('message', (e) => {
  if (e.data.type === 'notification') {
    showNotification(e.data.message)
  }
})
