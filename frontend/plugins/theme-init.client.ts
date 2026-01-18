export default defineNuxtPlugin(() => {
  // This plugin serves as a fallback in case the public script doesn't run
  // It runs after hydration is complete to ensure theme is properly applied
  if (process.client && typeof window !== 'undefined' && typeof localStorage !== 'undefined') {
    // Small delay to ensure hydration is complete
    setTimeout(() => {
      try {
        const savedTheme = localStorage.getItem('theme')
        const htmlElement = document.documentElement
        const isDarkClassPresent = htmlElement.classList.contains('dark')
        const shouldBeDark = savedTheme === 'dark'

        // Only apply changes if there's a mismatch
        if (shouldBeDark && !isDarkClassPresent) {
          htmlElement.classList.add('dark')
        } else if (!shouldBeDark && isDarkClassPresent) {
          htmlElement.classList.remove('dark')
        }

        // Clean up invalid theme values from localStorage
        if (savedTheme && !['light', 'dark', 'system'].includes(savedTheme)) {
          localStorage.removeItem('theme')
          htmlElement.classList.remove('dark')
        }
      } catch (e) {
        console.debug('Theme initialization fallback: error', e)
      }
    }, 0)
  }
})
