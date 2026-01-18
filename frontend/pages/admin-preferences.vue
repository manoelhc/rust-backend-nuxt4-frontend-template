<template>
  <NuxtLayout>
    <div class="space-y-4">
      <!-- Page Header -->
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
            {{ $t('pages.adminPreferences.title') }}
          </h1>
          <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
            {{ $t('pages.adminPreferences.description') }}
          </p>
        </div>
      </div>

      <!-- Logo Management Card -->
      <div class="p-6 bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700">
        <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-4">
          {{ $t('pages.adminPreferences.logoSettings') }}
        </h2>

        <div class="space-y-6">
          <!-- Logo Preview -->
          <div v-if="logoUrl" class="flex justify-center p-4 bg-gray-50 dark:bg-gray-900 rounded-lg">
            <img 
              :src="logoUrl" 
              :alt="logoAlt" 
              class="max-h-24 max-w-xs object-contain"
            />
          </div>
          <div v-else class="flex justify-center items-center p-4 bg-gray-50 dark:bg-gray-900 rounded-lg h-24">
            <p class="text-gray-400">{{ $t('pages.adminPreferences.noLogoPreview') }}</p>
          </div>

          <!-- Logo URL Input -->
          <div>
            <label class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
              {{ $t('pages.adminPreferences.logoUrl') }}
            </label>
            <input
              v-model="formData.logoUrl"
              type="url"
              :placeholder="$t('pages.adminPreferences.logoUrlPlaceholder')"
              class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
            />
            <p class="mt-1 text-xs text-gray-500 dark:text-gray-400">
              {{ $t('pages.adminPreferences.logoUrlInfo') }}
            </p>
          </div>

          <!-- Logo Alt Text Input -->
          <div>
            <label class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
              {{ $t('pages.adminPreferences.logoAltText') }}
            </label>
            <input
              v-model="formData.altText"
              type="text"
              :placeholder="$t('pages.adminPreferences.logoAltTextPlaceholder')"
              class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
            />
            <p class="mt-1 text-xs text-gray-500 dark:text-gray-400">
              {{ $t('pages.adminPreferences.logoAltTextInfo') }}
            </p>
          </div>

          <!-- Preview Update Button -->
          <button
            @click="updateLogoPreview"
            type="button"
            class="w-full px-4 py-2 bg-gray-200 text-gray-900 rounded-lg hover:bg-gray-300 transition-colors font-medium dark:bg-gray-700 dark:text-gray-100 dark:hover:bg-gray-600"
          >
            {{ $t('pages.adminPreferences.updatePreview') }}
          </button>
        </div>
      </div>

      <!-- Save Button -->
      <div class="flex justify-end gap-2">
        <button
          @click="cancelChanges"
          type="button"
          class="px-5 py-2.5 text-gray-900 bg-gray-300 hover:bg-gray-400 focus:ring-4 focus:outline-none focus:ring-gray-300 font-medium rounded-lg text-sm text-center dark:bg-gray-600 dark:text-white dark:hover:bg-gray-700 dark:focus:ring-gray-800"
        >
          {{ $t('pages.adminPreferences.cancel') }}
        </button>
        <button
          @click="saveChanges"
          type="button"
          :disabled="isSaving"
          class="px-5 py-2.5 text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm text-center disabled:opacity-50 disabled:cursor-not-allowed dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
        >
          {{ isSaving ? $t('pages.adminPreferences.saving') : $t('pages.adminPreferences.saveChanges') }}
        </button>
      </div>
    </div>
  </NuxtLayout>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

const { t } = useI18n()
const { post, get } = useApi()
const { success: showSuccessNotification, error: showErrorNotification } = useNotifications()

interface FormData {
  logoUrl: string
  altText: string
}

const formData = ref<FormData>({
  logoUrl: '',
  altText: 'Application Logo'
})

const logoUrl = ref('')
const logoAlt = ref('Application Logo')
const isSaving = ref(false)

// Load current logo settings on mount
onMounted(async () => {
  try {
    const response = await get<{ logo_url: string; alt_text: string }>('/admin/logo', 'adminLogo')
    if (response) {
      formData.value.logoUrl = response.logo_url || ''
      formData.value.altText = response.alt_text || 'Application Logo'
      if (response.logo_url) {
        logoUrl.value = response.logo_url
        logoAlt.value = response.alt_text || 'Application Logo'
      }
    }
  } catch (error) {
    console.error('Failed to load logo settings:', error)
    // Use defaults if API fails - this is expected during development
    formData.value.logoUrl = ''
    formData.value.altText = 'Application Logo'
  }
})

const updateLogoPreview = () => {
  if (formData.value.logoUrl) {
    logoUrl.value = formData.value.logoUrl
    logoAlt.value = formData.value.altText || 'Application Logo'
  }
}

const saveChanges = async () => {
  if (!formData.value.logoUrl) {
    showErrorNotification(t('pages.adminPreferences.logoUrlRequired'))
    return
  }

  if (!formData.value.altText) {
    showErrorNotification(t('pages.adminPreferences.altTextRequired'))
    return
  }

  isSaving.value = true

  try {
    await post('/admin/logo', {
      logo_url: formData.value.logoUrl,
      alt_text: formData.value.altText
    })

    showSuccessNotification(t('pages.adminPreferences.changesSaved'), 3000)
  } catch (error) {
    console.error('Failed to save logo:', error)
    showErrorNotification(t('pages.adminPreferences.saveError'))
  } finally {
    isSaving.value = false
  }
}

const cancelChanges = async () => {
  try {
    const response = await get<{ logo_url: string; alt_text: string }>('/admin/logo', 'adminLogo')
    if (response?.logo_url) {
      formData.value.logoUrl = response.logo_url
      formData.value.altText = response.alt_text
      logoUrl.value = response.logo_url
      logoAlt.value = response.alt_text
    }
  } catch (error) {
    console.error('Failed to reload logo settings:', error)
  }
}
</script>
