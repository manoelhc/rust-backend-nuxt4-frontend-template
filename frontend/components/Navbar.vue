<template>
  <nav class="fixed top-0 z-50 w-full bg-white border-b border-gray-200 dark:bg-gray-800 dark:border-gray-700">
    <div class="px-3 py-3 lg:px-5 lg:pl-3">
      <div class="flex items-center justify-between">
        <div class="flex items-center justify-start rtl:justify-end">
          <button
            @click="toggleSidebar"
            type="button"
            class="inline-flex items-center p-2 text-sm text-gray-500 rounded-lg sm:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600"
          >
            <span class="sr-only">Open sidebar</span>
            <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
              <path clip-rule="evenodd" fill-rule="evenodd" d="M2 4.75A.75.75 0 012.75 4h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 4.75zm0 10.5a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5h-7.5a.75.75 0 01-.75-.75zM2 10a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 10z"></path>
            </svg>
          </button>

          <!-- Logo or Project Name -->
          <div class="ml-2 flex items-center gap-2">
            <img
              v-if="logoUrl"
              :src="logoUrl"
              :alt="logoAlt"
              class="h-8 w-auto object-contain"
            />
            <span class="text-xl font-semibold sm:text-2xl whitespace-nowrap dark:text-white">
              {{ projectName }}
            </span>
          </div>
        </div>
        
        <div class="flex items-center gap-3">
          <!-- Language Selector -->
          <div class="relative">
            <button
              @click="toggleLangDropdown"
              type="button"
              class="inline-flex items-center px-3 py-2 text-sm font-medium text-center text-gray-900 bg-white border border-gray-300 rounded-lg hover:bg-gray-100 focus:ring-4 focus:outline-none focus:ring-gray-200 dark:bg-gray-800 dark:text-white dark:border-gray-600 dark:hover:bg-gray-700 dark:hover:border-gray-600 dark:focus:ring-gray-700"
            >
              <span class="fi" :class="`fi-${currentLocale === 'en' ? 'gb' : currentLocale}`"></span>
              <span class="ml-2">{{ currentLocaleName }}</span>
              <svg class="w-2.5 h-2.5 ml-2.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
              </svg>
            </button>
            
            <div
              v-if="showLangDropdown"
              class="absolute right-0 z-10 mt-2 w-48 bg-white divide-y divide-gray-100 rounded-lg shadow dark:bg-gray-700"
            >
              <ul class="py-2 text-sm text-gray-700 dark:text-gray-200">
                <li v-for="loc in locales" :key="loc.code">
                  <a
                    href="#"
                    @click.prevent="setLocaleAndSave(loc.code)"
                    class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white"
                  >
                    {{ loc.name }}
                  </a>
                </li>
              </ul>
            </div>
          </div>

          <!-- Dark Mode Toggle -->
          <button
            @click="toggleDarkMode"
            type="button"
            class="p-2 text-gray-500 rounded-lg hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600"
          >
            <svg v-if="!isDark" class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
              <path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z"></path>
            </svg>
            <svg v-else class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
              <path d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z"></path>
            </svg>
          </button>
        </div>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
const { locale, locales, setLocale } = useI18n()
const config = useRuntimeConfig()
const projectName = config.public.projectName
const { loadPreferences, saveTheme, saveLanguage, effectiveTheme } = usePreferences()
const { get } = useApi()

const showLangDropdown = ref(false)
const logoUrl = ref('')
const logoAlt = ref('Application Logo')

const currentLocale = computed(() => locale.value)
const currentLocaleName = computed(() => {
  const loc = locales.value.find((l: any) => l.code === locale.value)
  return loc?.name || 'English'
})

const isDark = ref(false)

// Update isDark only on client after hydration
watch(() => effectiveTheme.value, (newTheme) => {
  isDark.value = newTheme === 'dark'
}, { immediate: true })

onMounted(async () => {
  // Wait for hydration to complete
  await nextTick()

  // Load preferences (theme and language) only on client after hydration
  if (process.client) {
    loadPreferences()

    // Force a small delay to ensure DOM is fully updated
    await nextTick()

    // Trigger Flowbite reinitialization after theme is applied
    if (typeof (window as any).initFlowbite === 'function') {
      setTimeout(() => {
        ;(window as any).initFlowbite()
      }, 50)
    }
  }

  // Load navbar logo
  try {
    const response = await get<{ logo_url: string; alt_text: string }>('/admin/logo', 'navbarLogo')
    if (response) {
      if (response.logo_url) {
        logoUrl.value = response.logo_url
        logoAlt.value = response.alt_text || 'Application Logo'
      }
    }
  } catch (error) {
    // Silently fail - logo is optional and will use text fallback
    // This is expected during development when backend endpoint may not be available yet
  }

  // Close dropdown when clicking outside
  if (process.client) {
    document.addEventListener('click', (e) => {
      const target = e.target as HTMLElement
      if (!target.closest('.relative')) {
        showLangDropdown.value = false
      }
    })
  }
})

async function toggleDarkMode() {
  // Toggle between light and dark (not system)
  const newTheme = isDark.value ? 'light' : 'dark'
  saveTheme(newTheme)

  // Force Vue to re-evaluate computed properties
  await nextTick()
}

function toggleLangDropdown() {
  showLangDropdown.value = !showLangDropdown.value
}

function setLocaleAndSave(code: string) {
  setLocale(code)
  saveLanguage(code)
  showLangDropdown.value = false
}

function toggleSidebar() {
  const sidebar = document.getElementById('sidebar')
  if (sidebar) {
    sidebar.classList.toggle('-translate-x-full')
  }
}
</script>

<style scoped>
/* Flag icons would need the flag-icons library, simplified here */
.fi {
  width: 20px;
  height: 15px;
  display: inline-block;
  border-radius: 2px;
  background-size: cover;
}
</style>
