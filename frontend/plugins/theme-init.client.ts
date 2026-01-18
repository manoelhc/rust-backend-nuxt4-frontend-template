export default defineNuxtPlugin(() => {
  // This plugin runs only on client and before Nuxt hydration
  // It initializes the theme class on the html element based on localStorage
  if (process.client && typeof window !== 'undefined' && typeof localStorage !== 'undefined') {
    const savedTheme = localStorage.getItem('theme')
    const htmlElement = document.documentElement

    // Apply theme class based on saved preference
    if (savedTheme === 'dark') {
      htmlElement.classList.add('dark')
    } else {
      // For 'light' or no saved theme, ensure dark class is removed
      htmlElement.classList.remove('dark')
    }

    // Clean up invalid theme values from localStorage
    if (savedTheme && !['light', 'dark', 'system'].includes(savedTheme)) {
      localStorage.removeItem('theme')
      htmlElement.classList.remove('dark')
    }
  }
})
