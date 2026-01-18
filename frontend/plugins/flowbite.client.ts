export default defineNuxtPlugin(() => {
  // Configure Flowbite to use class-based dark mode
  if (process.client && typeof window !== 'undefined') {
    // Wait for Flowbite to load
    const checkFlowbite = setInterval(() => {
      if (typeof window !== 'undefined' && (window as any).initFlowbite) {
        clearInterval(checkFlowbite)
        
        // Configure Flowbite's dark mode to use 'class' strategy
        const htmlElement = document.documentElement
        const isDark = htmlElement.classList.contains('dark')
        
        // Tell Flowbite to respect the class-based dark mode
        if (isDark) {
          htmlElement.setAttribute('data-theme', 'dark')
        } else {
          htmlElement.setAttribute('data-theme', 'light')
        }
        
        // Re-initialize Flowbite with the current theme
        ;(window as any).initFlowbite()
      }
    }, 100)
    
    // Clear interval after 2 seconds to prevent infinite loop
    setTimeout(() => clearInterval(checkFlowbite), 2000)
  }
})
