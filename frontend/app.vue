<template>
  <div>
    <NuxtPage />
  </div>
</template>

<script setup lang="ts">
// Debug script to check initial theme state
onMounted(() => {
  if (process.client) {
    console.log('[app.vue] mounted, checking theme state')
    const savedTheme = localStorage.getItem('theme')
    console.log('[app.vue] localStorage theme:', savedTheme)
    console.log('[app.vue] html.classList:', Array.from(document.documentElement.classList))
    console.log('[app.vue] system prefers dark:', window.matchMedia('(prefers-color-scheme: dark)').matches)

    // Validate theme value - ensure it's one of the valid options
    if (savedTheme && !['light', 'dark', 'system'].includes(savedTheme)) {
      console.warn('[app.vue] Invalid theme value in localStorage, clearing it:', savedTheme)
      localStorage.removeItem('theme')
    }
  }
})
</script>
