// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2024-11-01',
  devtools: { enabled: true },
  
  modules: [
    '@nuxtjs/tailwindcss',
    '@nuxtjs/i18n'
  ],

  runtimeConfig: {
    public: {
      apiUrl: process.env.NUXT_PUBLIC_API_URL || 'http://localhost:3000',
      projectName: process.env.NUXT_PUBLIC_PROJECT_NAME || 'My Application',
      aiFrontendDev: process.env.AI_FRONTEND_DEV || 'false'
    }
  },

  i18n: {
    locales: [
      { code: 'en', file: 'en.json', name: 'English' },
      { code: 'pt', file: 'pt.json', name: 'Português' },
      { code: 'es', file: 'es.json', name: 'Español' }
    ],
    defaultLocale: 'en',
    lazy: true,
    langDir: 'locales',
    strategy: 'no_prefix',
    detectBrowserLanguage: {
      useCookie: true,
      cookieKey: 'i18n_redirected',
      redirectOn: 'root'
    }
  },

  css: ['~/assets/css/main.css'],

  app: {
    head: {
      title: 'My Application',
      meta: [
        { charset: 'utf-8' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' }
      ],
      script: [
        {
          src: 'https://cdn.jsdelivr.net/npm/flowbite@2.5.0/dist/flowbite.min.js',
          defer: true
        }
      ]
    }
  }
})
