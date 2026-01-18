export default {
  darkMode: 'class',
  content: [
    './components/**/*.{js,vue,ts}',
    './layouts/**/*.{js,vue,ts}',
    './pages/**/*.{js,vue,ts}',
    './plugins/**/*.{js,ts}',
    './app.vue',
    './assets/**/*.{js,css}',
    './node_modules/flowbite/**/*.{js,ts}'
  ],
  theme: {
    extend: {}
  },
  plugins: [
    require('flowbite/plugin')
  ]
}
