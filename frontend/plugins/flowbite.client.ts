export default defineNuxtPlugin(() => {
  if (process.client && typeof window !== 'undefined') {
    // Wait for Flowbite script to load and initialize
    const checkFlowbite = setInterval(() => {
      if (typeof (window as any).initFlowbite === 'function') {
        clearInterval(checkFlowbite)
        // Initialize Flowbite with initial dark mode setting
        ;(window as any).initFlowbite()
      }
    }, 100)

    setTimeout(() => clearInterval(checkFlowbite), 3000)

    // Watch for dark class changes and reinitialize Flowbite
    const observer = new MutationObserver((mutations) => {
      mutations.forEach((mutation) => {
        if (mutation.type === 'attributes' && mutation.attributeName === 'class') {
          // Force style recalculation by triggering a reflow
          // This ensures browsers repaint all elements with new styles
          const html = document.documentElement
          const trigger = html.offsetHeight
          
          // Re-initialize Flowbite dropdowns, modals, etc. when theme changes
          if (typeof (window as any).initFlowbite === 'function') {
            // Small delay to allow Tailwind to apply changes
            setTimeout(() => {
              ;(window as any).initFlowbite()
            }, 50)
          }
        }
      })
    })

    // Start observing the html element for class changes
    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ['class']
    })
  }
})
