<template>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-900">
    <Navbar />
    <Sidebar />
    
    <!-- Main content -->
    <div class="sm:ml-64">
      <div class="p-4 mt-14">
        <slot />
      </div>
      
      <!-- Footer -->
      <footer class="bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700 p-4 text-center text-sm text-gray-500 dark:text-gray-400">
        <div class="flex items-center justify-center gap-2">
          <span>{{ $t('common.version') }}: {{ version }}</span>
        </div>
      </footer>
    </div>
  </div>
</template>

<script setup lang="ts">
const version = ref('0.1.0')
const config = useRuntimeConfig()

// Fetch version from backend
onMounted(async () => {
  try {
    const response = await fetch(`${config.public.apiUrl}/system/version`)
    const data = await response.json()
    version.value = data.version
  } catch (error) {
    console.error('Failed to fetch version:', error)
  }
})
</script>
