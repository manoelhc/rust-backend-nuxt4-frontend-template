import { mockData, createMockResponse } from '~/utils/mockData'

/**
 * Composable for making API calls with automatic mock support when AI_FRONTEND_DEV is enabled.
 * This allows AI agents and UI builders to work without a running backend.
 * 
 * Automatically includes JWT token from 'auth-token' cookie when available.
 */
export const useApi = () => {
  const config = useRuntimeConfig()
  const isDevMode = config.public.aiFrontendDev === 'true' || config.public.aiFrontendDev === true
  
  /**
   * Get the JWT token from cookie if available
   */
  const getAuthToken = () => {
    const cookie = useCookie('auth-token')
    return cookie.value || null
  }
  
  /**
   * Get common headers including auth token if available
   */
  const getHeaders = () => {
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
    }
    
    const token = getAuthToken()
    if (token) {
      headers['Authorization'] = `Bearer ${token}`
    }
    
    return headers
  }
  
  /**
   * Make a GET request to the API or return mock data
   */
  const get = async <T>(endpoint: string, mockKey?: keyof typeof mockData): Promise<T> => {
    if (isDevMode && mockKey && mockData[mockKey]) {
      console.log(`[AI_FRONTEND_DEV] Returning mock data for: ${endpoint}`)
      return createMockResponse(mockData[mockKey] as T)
    }
    
    try {
      const response = await fetch(`${config.public.apiUrl}${endpoint}`, {
        headers: getHeaders()
      })
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`)
      }
      return await response.json()
    } catch (error) {
      console.error(`API call failed for ${endpoint}:`, error)
      
      // Fallback to mock data if available and real call fails
      if (mockKey && mockData[mockKey]) {
        console.warn(`[Fallback] Using mock data for: ${endpoint}`)
        return createMockResponse(mockData[mockKey] as T, 100)
      }
      
      throw error
    }
  }
  
  /**
   * Make a POST request to the API or return mock data
   */
  const post = async <T>(
    endpoint: string,
    body?: any,
    mockKey?: keyof typeof mockData
  ): Promise<T> => {
    if (isDevMode && mockKey && mockData[mockKey]) {
      console.log(`[AI_FRONTEND_DEV] Returning mock data for: ${endpoint}`)
      return createMockResponse(mockData[mockKey] as T)
    }
    
    try {
      const response = await fetch(`${config.public.apiUrl}${endpoint}`, {
        method: 'POST',
        headers: getHeaders(),
        body: body ? JSON.stringify(body) : undefined,
      })
      
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`)
      }
      return await response.json()
    } catch (error) {
      console.error(`API call failed for ${endpoint}:`, error)
      
      // Fallback to mock data if available and real call fails
      if (mockKey && mockData[mockKey]) {
        console.warn(`[Fallback] Using mock data for: ${endpoint}`)
        return createMockResponse(mockData[mockKey] as T, 100)
      }
      
      throw error
    }
  }
  
  /**
   * Make an authenticated request with JWT token
   * @deprecated Use get() or post() instead - they now automatically include the auth token from cookie
   */
  const authenticatedRequest = async <T>(
    endpoint: string,
    token: string,
    method: 'GET' | 'POST' = 'GET',
    body?: any,
    mockKey?: keyof typeof mockData
  ): Promise<T> => {
    if (isDevMode && mockKey && mockData[mockKey]) {
      console.log(`[AI_FRONTEND_DEV] Returning mock data for: ${endpoint}`)
      return createMockResponse(mockData[mockKey] as T)
    }
    
    try {
      const options: RequestInit = {
        method,
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`,
        },
      }
      
      if (body && method === 'POST') {
        options.body = JSON.stringify(body)
      }
      
      const response = await fetch(`${config.public.apiUrl}${endpoint}`, options)
      
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`)
      }
      return await response.json()
    } catch (error) {
      console.error(`Authenticated API call failed for ${endpoint}:`, error)
      
      // Fallback to mock data if available and real call fails
      if (mockKey && mockData[mockKey]) {
        console.warn(`[Fallback] Using mock data for: ${endpoint}`)
        return createMockResponse(mockData[mockKey] as T, 100)
      }
      
      throw error
    }
  }
  
  return {
    get,
    post,
    authenticatedRequest,
    getAuthToken,
    isDevMode
  }
}
