<template>
  <div>
    <NuxtPage />
  </div>
</template>

<script setup lang="ts">
// Initialize theme BEFORE rendering to prevent flash
// This runs immediately when the script is evaluated
if (process.client && typeof window !== 'undefined') {
  const savedTheme = localStorage.getItem('theme')
  const htmlElement = document.documentElement

  // Explicitly set the theme class
  if (savedTheme === 'dark') {
    htmlElement.classList.add('dark')
  } else {
    // For 'light' or no saved theme, ensure dark class is removed
    htmlElement.classList.remove('dark')
  }
}

// Validate and clean up theme on mount
onMounted(() => {
  if (process.client) {
    const savedTheme = localStorage.getItem('theme')

    // Clear invalid theme values from localStorage
    if (savedTheme && !['light', 'dark', 'system'].includes(savedTheme)) {
      localStorage.removeItem('theme')
      document.documentElement.classList.remove('dark')
    }
  }
})
</script>
