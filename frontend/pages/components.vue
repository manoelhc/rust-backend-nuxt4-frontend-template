<template>
  <NuxtLayout>
    <div class="space-y-8">
      <!-- Page Header -->
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
            {{ $t('pages.components.title') }}
          </h1>
          <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
            {{ $t('pages.components.description') }}
          </p>
        </div>
      </div>

      <!-- Notifications Section -->
      <section class="space-y-4">
        <div>
          <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">
            {{ $t('pages.components.notifications.title') }}
          </h2>
          <p class="text-sm text-gray-600 dark:text-gray-400">
            {{ $t('pages.components.notifications.description') }}
          </p>
        </div>

        <div class="p-6 bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700">
          <div class="space-y-3">
            <button
              @click="showSuccessNotification"
              type="button"
              class="w-full sm:w-auto px-5 py-2.5 text-white bg-green-700 hover:bg-green-800 focus:ring-4 focus:outline-none focus:ring-green-300 font-medium rounded-lg text-sm text-center dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800"
            >
              {{ $t('pages.components.notifications.showSuccess') }}
            </button>
            <button
              @click="showErrorNotification"
              type="button"
              class="w-full sm:w-auto px-5 py-2.5 text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:outline-none focus:ring-red-300 font-medium rounded-lg text-sm text-center dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-800"
            >
              {{ $t('pages.components.notifications.showError') }}
            </button>
            <button
              @click="showWarningNotification"
              type="button"
              class="w-full sm:w-auto px-5 py-2.5 text-white bg-yellow-700 hover:bg-yellow-800 focus:ring-4 focus:outline-none focus:ring-yellow-300 font-medium rounded-lg text-sm text-center dark:bg-yellow-600 dark:hover:bg-yellow-700 dark:focus:ring-yellow-800"
            >
              {{ $t('pages.components.notifications.showWarning') }}
            </button>
            <button
              @click="showInfoNotification"
              type="button"
              class="w-full sm:w-auto px-5 py-2.5 text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
            >
              {{ $t('pages.components.notifications.showInfo') }}
            </button>
          </div>

          <div class="mt-4 p-4 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
            <p class="text-xs text-gray-500 dark:text-gray-400 font-mono">
              {{ $t('pages.components.notifications.tryPrompt') }}
            </p>
            <div class="mt-2 space-y-1 text-xs text-gray-600 dark:text-gray-300 font-mono">
              <p>&lt;const { success, error, info, warning } = useNotifications()</p>
              <p>&lt;success('Message here')</p>
            </div>
          </div>
        </div>
      </section>

      <!-- Empty State Section -->
      <section class="space-y-4">
        <div>
          <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">
            {{ $t('pages.components.upcoming.title') }}
          </h2>
          <p class="text-sm text-gray-600 dark:text-gray-400">
            {{ $t('pages.components.upcoming.description') }}
          </p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="p-6 bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700 text-center">
            <svg class="w-12 h-12 text-gray-300 dark:text-gray-600 mx-auto mb-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
            </svg>
            <p class="text-gray-700 dark:text-gray-300 font-medium">
              {{ $t('pages.components.upcoming.comingSoon') }}
            </p>
          </div>
        </div>
      </section>

      <!-- Component Guidelines -->
      <section class="space-y-4">
        <div>
          <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">
            {{ $t('pages.components.guidelines.title') }}
          </h2>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="p-4 bg-blue-50 border border-blue-200 rounded-lg dark:bg-gray-800 dark:border-blue-900">
            <h3 class="font-semibold text-blue-900 dark:text-blue-400 mb-2">{{ $t('pages.components.guidelines.accessibility') }}</h3>
            <p class="text-sm text-blue-800 dark:text-blue-300">
              {{ $t('pages.components.guidelines.accessibilityDesc') }}
            </p>
          </div>
          <div class="p-4 bg-green-50 border border-green-200 rounded-lg dark:bg-gray-800 dark:border-green-900">
            <h3 class="font-semibold text-green-900 dark:text-green-400 mb-2">{{ $t('pages.components.guidelines.dark') }}</h3>
            <p class="text-sm text-green-800 dark:text-green-300">
              {{ $t('pages.components.guidelines.darkDesc') }}
            </p>
          </div>
          <div class="p-4 bg-purple-50 border border-purple-200 rounded-lg dark:bg-gray-800 dark:border-purple-900">
            <h3 class="font-semibold text-purple-900 dark:text-purple-400 mb-2">{{ $t('pages.components.guidelines.responsive') }}</h3>
            <p class="text-sm text-purple-800 dark:text-purple-300">
              {{ $t('pages.components.guidelines.responsiveDesc') }}
            </p>
          </div>
          <div class="p-4 bg-orange-50 border border-orange-200 rounded-lg dark:bg-gray-800 dark:border-orange-900">
            <h3 class="font-semibold text-orange-900 dark:text-orange-400 mb-2">{{ $t('pages.components.guidelines.i18n') }}</h3>
            <p class="text-sm text-orange-800 dark:text-orange-300">
              {{ $t('pages.components.guidelines.i18nDesc') }}
            </p>
          </div>
        </div>
      </section>
    </div>
  </NuxtLayout>
</template>

<script setup lang="ts">
const { t } = useI18n()
const { success, error, warning, info } = useNotifications()

const showSuccessNotification = () => {
  success(t('pages.components.notifications.sampleSuccess'), 5000)
}

const showErrorNotification = () => {
  error(t('pages.components.notifications.sampleError'), 5000)
}

const showWarningNotification = () => {
  warning(t('pages.components.notifications.sampleWarning'), 5000)
}

const showInfoNotification = () => {
  info(t('pages.components.notifications.sampleInfo'), 5000)
}
</script>
