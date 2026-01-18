// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2024-11-01',
  devtools: { enabled: false },
  
  modules: [
    '@nuxtjs/i18n'
  ],

  postcss: {
    plugins: {
      '@tailwindcss/postcss': {}
    }
  },

  devServer: {
    port: parseInt(process.env.NUXT_PUBLIC_APP_PORT || '3000')
  },

  nitro: {
    port: parseInt(process.env.NUXT_PUBLIC_APP_PORT || '3000')
  },

  runtimeConfig: {
    public: {
      apiUrl: process.env.NUXT_PUBLIC_API_URL || 'http://nginx:8070/api',
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
    langDir: '../locales',
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
          // This script runs immediately before Vue hydration
          // It prevents hydration mismatches by not applying theme class during SSR
          innerHTML: `
            if (typeof localStorage !== 'undefined' && typeof document !== 'undefined') {
              const theme = localStorage.getItem('theme');
              if (theme === 'dark') {
                document.documentElement.classList.add('dark');
              } else {
                document.documentElement.classList.remove('dark');
              }
            }
          `,
          type: 'text/javascript'
        },
        {
          src: 'https://cdn.jsdelivr.net/npm/flowbite@2.5.0/dist/flowbite.min.js',
          defer: true
        }
      ]
    }
  }
})
