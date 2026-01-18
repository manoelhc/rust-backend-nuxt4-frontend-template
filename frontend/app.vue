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
    console.log('[app.vue] localStorage theme:', localStorage.getItem('theme'))
    console.log('[app.vue] html.classList:', Array.from(document.documentElement.classList))
    console.log('[app.vue] system prefers dark:', window.matchMedia('(prefers-color-scheme: dark)').matches)
  }
})
</script>
