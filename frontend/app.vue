<template>
  <div>
    <NuxtPage />
  </div>
</template>

<script setup lang="ts">
// Initialize theme BEFORE rendering to prevent flash
// This runs during SSR and client mount
if (process.client) {
  const savedTheme = localStorage.getItem('theme')

  // If no saved theme, start with light mode
  if (!savedTheme) {
    // Remove any dark class that might have been set by system preference
    document.documentElement.classList.remove('dark')
  }
}

// Validate theme value on mount
onMounted(() => {
  if (process.client) {
    const savedTheme = localStorage.getItem('theme')

    // Clear invalid theme values from localStorage
    if (savedTheme && !['light', 'dark', 'system'].includes(savedTheme)) {
      localStorage.removeItem('theme')
    }
  }
})
</script>
