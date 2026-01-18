import { ref, computed } from 'vue'

export type NotificationType = 'success' | 'error' | 'info' | 'warning'

export interface Notification {
  id: string
  message: string
  type: NotificationType
  duration?: number // in milliseconds, undefined = sticky
  timestamp: number
}

// Shared notifications state
const notifications = ref<Notification[]>([])
const STORAGE_KEY = 'app_notifications'
const AUTO_DISMISS_DURATION = 5000 // 5 seconds

// Load notifications from localStorage
const loadNotificationsFromStorage = () => {
  if (process.client && typeof localStorage !== 'undefined') {
    try {
      const stored = localStorage.getItem(STORAGE_KEY)
      if (stored) {
        const parsed = JSON.parse(stored) as Notification[]
        notifications.value = parsed
        // Clear storage after loading
        localStorage.removeItem(STORAGE_KEY)
      }
    } catch (e) {
      console.debug('Failed to load notifications from storage:', e)
    }
  }
}

// Save notifications to localStorage
const saveNotificationsToStorage = () => {
  if (process.client && typeof localStorage !== 'undefined') {
    try {
      if (notifications.value.length > 0) {
        localStorage.setItem(STORAGE_KEY, JSON.stringify(notifications.value))
      }
    } catch (e) {
      console.debug('Failed to save notifications to storage:', e)
    }
  }
}

// Setup page exit listener to save notifications
const setupExitListener = () => {
  if (process.client && typeof window !== 'undefined') {
    window.addEventListener('beforeunload', () => {
      saveNotificationsToStorage()
    })
  }
}

export const useNotifications = () => {
  // Generate unique ID for notification
  const generateId = () => `notification_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`

  // Add a new notification
  const addNotification = (
    message: string,
    type: NotificationType = 'info',
    duration?: number
  ) => {
    const id = generateId()
    const notification: Notification = {
      id,
      message,
      type,
      duration: duration ?? AUTO_DISMISS_DURATION,
      timestamp: Date.now()
    }

    notifications.value.push(notification)

    // Auto-dismiss if duration is specified
    if (notification.duration) {
      setTimeout(() => {
        removeNotification(id)
      }, notification.duration)
    }

    return id
  }

  // Remove a notification
  const removeNotification = (id: string) => {
    const index = notifications.value.findIndex((n) => n.id === id)
    if (index > -1) {
      notifications.value.splice(index, 1)
    }
  }

  // Show success notification
  const success = (message: string, duration?: number) => {
    return addNotification(message, 'success', duration)
  }

  // Show error notification
  const error = (message: string, duration?: number) => {
    return addNotification(message, 'error', duration)
  }

  // Show info notification
  const info = (message: string, duration?: number) => {
    return addNotification(message, 'info', duration)
  }

  // Show warning notification
  const warning = (message: string, duration?: number) => {
    return addNotification(message, 'warning', duration)
  }

  // Clear all notifications
  const clearAll = () => {
    notifications.value = []
  }

  // Load notifications on first use
  if (process.client && !notifications.value.length) {
    loadNotificationsFromStorage()
    setupExitListener()
  }

  return {
    notifications: computed(() => notifications.value),
    addNotification,
    removeNotification,
    success,
    error,
    info,
    warning,
    clearAll,
    saveNotificationsToStorage
  }
}
