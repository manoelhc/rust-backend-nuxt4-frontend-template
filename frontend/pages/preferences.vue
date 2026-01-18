<template>
  <NuxtLayout>
    <div class="space-y-4">
      <!-- Page Header -->
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
            {{ $t('pages.preferences.title') }}
          </h1>
          <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
            {{ $t('pages.preferences.description') }}
          </p>
        </div>
      </div>


      <!-- User Information Card -->
      <div class="p-6 bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700">
        <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-4">
          {{ $t('pages.preferences.userInfo') }}
        </h2>
        
        <div class="space-y-4">
          <div>
            <label class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
              {{ $t('pages.preferences.name') }}
            </label>
            <div class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:text-white">
              {{ userName }}
            </div>
          </div>
          
          <div>
            <label class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
              {{ $t('pages.preferences.email') }}
            </label>
            <div class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:text-white">
              {{ userEmail }}
            </div>
          </div>
        </div>
      </div>

      <!-- Language Settings Card -->
      <div class="p-6 bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700">
        <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-4">
          {{ $t('pages.preferences.languageSettings') }}
        </h2>
        
        <div class="space-y-4">
          <div>
            <label for="language" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
              {{ $t('pages.preferences.preferredLanguage') }}
            </label>
            <p class="text-xs text-gray-500 dark:text-gray-400 mb-2">
              {{ $t('pages.preferences.languageDescription') }}
            </p>
            <select
              id="language"
              v-model="selectedLanguage"
              class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
            >
              <option v-for="loc in availableLocales" :key="loc.code" :value="loc.code">
                {{ loc.name }}
              </option>
            </select>
          </div>
        </div>
      </div>

      <!-- Theme Settings Card -->
      <div class="p-6 bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700">
        <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-4">
          {{ $t('pages.preferences.themeSettings') }}
        </h2>
        
        <div class="space-y-4">
          <div>
            <label for="theme" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
              {{ $t('pages.preferences.preferredTheme') }}
            </label>
            <p class="text-xs text-gray-500 dark:text-gray-400 mb-2">
              {{ $t('pages.preferences.themeDescription') }}
            </p>
            <select
              id="theme"
              v-model="selectedTheme"
              class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
            >
              <option value="light">{{ $t('pages.preferences.themeLight') }}</option>
              <option value="dark">{{ $t('pages.preferences.themeDark') }}</option>
              <option value="system">{{ $t('pages.preferences.themeSystem') }}</option>
            </select>
          </div>
        </div>
      </div>

      <!-- Save Button -->
      <div class="flex justify-end">
        <button
          @click="saveChanges"
          type="button"
          class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
        >
          {{ $t('pages.preferences.saveChanges') }}
        </button>
      </div>
    </div>
  </NuxtLayout>
</template>

<script setup lang="ts">
interface UserProfile {
  user: {
    id: string
    sub: string
    user_email: string
    user_fullname: string
    organization?: string
    properties: any
    created_at: string
    updated_at: string
  }
}

const { locale, locales } = useI18n()
const { get } = useApi()
const { savePreferences, loadPreferences, theme, savedLanguage } = usePreferences()
const { success: showSuccessNotification } = useNotifications()

// User information
const userName = ref('John Doe')
const userEmail = ref('user@example.com')

// Preferences state
const selectedLanguage = ref(locale.value)
const selectedTheme = ref<'light' | 'dark' | 'system'>('system')

// Available locales for the select
const availableLocales = computed(() => {
  return locales.value as Array<{ code: string; name: string }>
})

// Load user profile and preferences on mount
onMounted(async () => {
  // Load saved preferences
  loadPreferences()
  
  // Set the form values from loaded preferences
  selectedLanguage.value = savedLanguage.value
  selectedTheme.value = theme.value
  
  // Try to load user profile from API
  try {
    const profileData = await get<UserProfile>('/profile', 'profile')
    if (profileData?.user) {
      userName.value = profileData.user.user_fullname || 'John Doe'
      userEmail.value = profileData.user.user_email || 'user@example.com'
    }
  } catch (error) {
    console.error('Failed to load user profile:', error)
    // Keep default values if API call fails
  }
})

// Save changes handler
const saveChanges = () => {
  savePreferences({
    theme: selectedTheme.value,
    language: selectedLanguage.value
  })
  
  // Show success message
  showSuccessMessage.value = true
  
  // Hide success message after 3 seconds
  setTimeout(() => {
    showSuccessMessage.value = false
  }, 3000)
}
</script>
