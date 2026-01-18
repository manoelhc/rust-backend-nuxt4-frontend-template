import { ref, computed } from 'vue'

export type Theme = 'light' | 'dark' | 'system'

interface UserPreferences {
  theme: Theme
  language: string
}

const DARK_MODE_MEDIA_QUERY = '(prefers-color-scheme: dark)'

// Shared state across all component instances
// Initialize with light theme by default
const theme = ref<Theme>('light')
const savedLanguage = ref<string>('en')

// MediaQueryList instance for system theme detection
let mediaQueryList: MediaQueryList | null = null

// Initialize media query list
const initMediaQuery = () => {
  if (process.client && !mediaQueryList) {
    mediaQueryList = window.matchMedia(DARK_MODE_MEDIA_QUERY)

    // Listen for system theme changes
    mediaQueryList.addEventListener('change', (e) => {
      if (theme.value === 'system') {
        applyTheme('system')
      }
    })
  }
}

// Apply theme to document - module level function
const applyTheme = (selectedTheme: Theme) => {
  if (process.client) {
    const root = document.documentElement
    console.log('[applyTheme] selectedTheme:', selectedTheme)

    if (selectedTheme === 'system') {
      initMediaQuery()
      const systemPrefersDark = mediaQueryList?.matches ?? false
      console.log('[applyTheme] system theme, systemPrefersDark:', systemPrefersDark)
      if (systemPrefersDark) {
        root.classList.add('dark')
      } else {
        root.classList.remove('dark')
      }
    } else if (selectedTheme === 'dark') {
      console.log('[applyTheme] adding dark class')
      root.classList.add('dark')
    } else if (selectedTheme === 'light') {
      console.log('[applyTheme] removing dark class')
      root.classList.remove('dark')
    }
    console.log('[applyTheme] html.classList after:', Array.from(root.classList))
  }
}

export const usePreferences = () => {
  const { locale, setLocale } = useI18n()

  // Load preferences from localStorage
  const loadPreferences = () => {
    if (process.client) {
      // Load theme preference
      const savedTheme = localStorage.getItem('theme') as Theme
      console.log('[loadPreferences] savedTheme from localStorage:', savedTheme)

      if (savedTheme && ['light', 'dark', 'system'].includes(savedTheme)) {
        console.log('[loadPreferences] setting theme to:', savedTheme)
        theme.value = savedTheme
      } else if (!savedTheme) {
        // First time user - ensure we remove dark class
        console.log('[loadPreferences] no saved theme, defaulting to light')
        theme.value = 'light'
        // Explicitly remove dark class to ensure light mode is applied
        document.documentElement.classList.remove('dark')
      } else {
        console.log('[loadPreferences] invalid saved theme, resetting to light:', savedTheme)
        theme.value = 'light'
        localStorage.removeItem('theme')
      }

      // Load language preference
      const savedLang = localStorage.getItem('language')
      if (savedLang) {
        savedLanguage.value = savedLang
        setLocale(savedLang)
      }

      // Apply theme
      console.log('[loadPreferences] applying theme:', theme.value)
      applyTheme(theme.value)
    }
  }
  
  // Save theme preference
  const saveTheme = (newTheme: Theme) => {
    console.log('[saveTheme] called with:', newTheme, 'old theme.value:', theme.value)
    theme.value = newTheme
    if (process.client) {
      localStorage.setItem('theme', newTheme)
      console.log('[saveTheme] saved to localStorage, calling applyTheme')
      applyTheme(newTheme)
    }
  }
  
  // Save language preference
  const saveLanguage = (newLanguage: string) => {
    if (process.client) {
      savedLanguage.value = newLanguage
      localStorage.setItem('language', newLanguage)
      setLocale(newLanguage)
    }
  }
  
  // Save all preferences
  const savePreferences = (preferences: UserPreferences) => {
    saveTheme(preferences.theme)
    saveLanguage(preferences.language)
  }
  
  // Get current effective theme (resolves 'system' to actual theme)
  const effectiveTheme = computed(() => {
    if (theme.value === 'system') {
      if (process.client) {
        initMediaQuery()
        return mediaQueryList?.matches ? 'dark' : 'light'
      }
      return 'light'
    }
    return theme.value
  })

  return {
    theme,
    savedLanguage,
    effectiveTheme,
    loadPreferences,
    saveTheme,
    saveLanguage,
    savePreferences,
    applyTheme
  }
}
