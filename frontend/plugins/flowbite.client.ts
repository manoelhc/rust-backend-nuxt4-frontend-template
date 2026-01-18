export default defineNuxtPlugin(() => {
  if (process.client && typeof window !== 'undefined') {
    // Watch for dark class changes and reinitialize Flowbite
    const observer = new MutationObserver((mutations) => {
      mutations.forEach((mutation) => {
        if (mutation.type === 'attributes' && mutation.attributeName === 'class') {
          // Re-initialize Flowbite dropdowns, modals, etc. when theme changes
          if (typeof (window as any).initFlowbite === 'function') {
            ;(window as any).initFlowbite()
          }
        }
      })
    })

    // Start observing the html element for class changes
    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ['class']
    })

    // Wait for Flowbite script to load and initialize
    const checkFlowbite = setInterval(() => {
      if (typeof (window as any).initFlowbite === 'function') {
        clearInterval(checkFlowbite)
        ;(window as any).initFlowbite()
      }
    }, 100)

    setTimeout(() => clearInterval(checkFlowbite), 3000)
  }
})
