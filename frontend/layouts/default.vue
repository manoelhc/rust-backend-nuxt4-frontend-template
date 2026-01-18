<template>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-900">
    <ClientOnly>
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
    </ClientOnly>
  </div>
</template>

<script setup lang="ts">
const version = ref('0.1.0')
const { get } = useApi()

// Fetch version from backend or mock
onMounted(async () => {
  try {
    const data = await get<{ version: string }>('/system/version', 'version')
    version.value = data.version
  } catch (error) {
    console.error('Failed to fetch version:', error)
  }
})
</script>
