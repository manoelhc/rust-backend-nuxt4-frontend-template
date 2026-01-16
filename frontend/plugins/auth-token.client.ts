/**
 * Plugin to handle JWT token from URL query parameter
 * When NUXT_PUBLIC_ENABLE_URL_AUTH is enabled, this plugin:
 * 1. Checks for 'auth-token' query parameter in the URL
 * 2. Stores it in a cookie with the same name
 * 3. Removes the query parameter from the URL for security
 * 4. Redirects to the clean URL
 */
export default defineNuxtPlugin(() => {
  const config = useRuntimeConfig()
  const route = useRoute()
  
  // Check if URL auth is enabled
  const enableUrlAuth = config.public.enableUrlAuth === 'true' || config.public.enableUrlAuth === true
  
  if (!enableUrlAuth) {
    return
  }
  
  // Check for auth-token in URL query parameters
  const authToken = route.query['auth-token'] as string | undefined
  
  if (authToken) {
    // Store the token in a cookie
    const cookie = useCookie('auth-token', {
      maxAge: 60 * 60 * 24 * 7, // 7 days
      path: '/',
      sameSite: 'lax',
      secure: process.env.NODE_ENV === 'production'
    })
    
    cookie.value = authToken
    
    console.log('[Auth] JWT token received from URL and stored in cookie')
    
    // Remove the auth-token from URL query parameters for security
    const newQuery = { ...route.query }
    delete newQuery['auth-token']
    
    // Redirect to the same page without the auth-token parameter
    navigateTo({
      path: route.path,
      query: newQuery
    })
  }
})
