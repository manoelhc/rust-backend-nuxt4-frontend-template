// Theme initialization script
// This runs immediately on page load, before Vue hydration
// to prevent hydration mismatches
(function() {
  if (typeof localStorage !== 'undefined' && typeof document !== 'undefined') {
    try {
      const theme = localStorage.getItem('theme');
      const html = document.documentElement;
      
      if (theme === 'dark') {
        html.classList.add('dark');
      } else {
        html.classList.remove('dark');
      }
    } catch (e) {
      // Silently fail if localStorage is not available
      console.debug('Theme initialization: localStorage not available');
    }
  }
})();
