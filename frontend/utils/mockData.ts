// Mock data for AI frontend development
export const mockData = {
  version: {
    version: '0.1.0'
  },
  
  health: {
    status: 'ok',
    message: 'Service is healthy'
  },
  
  uptime: {
    uptime_seconds: 86400,
    uptime_formatted: '1d 0h 0m 0s'
  },
  
  validateToken: {
    valid: true,
    message: 'Token is valid'
  },
  
  onboarding: {
    user_id: '550e8400-e29b-41d4-a716-446655440000',
    message: 'User registered successfully',
    is_new_user: true
  },
  
  profile: {
    user: {
      id: '550e8400-e29b-41d4-a716-446655440000',
      sub: 'user123',
      user_email: 'user@example.com',
      user_fullname: 'John Doe',
      organization: 'Acme Corp',
      properties: {
        department: 'Engineering',
        role: 'Developer'
      },
      created_at: '2026-01-01T00:00:00Z',
      updated_at: '2026-01-16T00:00:00Z'
    }
  },

  adminLogo: {
    logo_url: '',
    alt_text: 'Application Logo'
  },

  navbarLogo: {
    logo_url: '',
    alt_text: 'Application Logo'
  }
}

// Simulate network delay for realistic mock behavior
export const mockDelay = (ms: number = 300) => {
  return new Promise(resolve => setTimeout(resolve, ms))
}

// Mock API response generator
export const createMockResponse = async <T>(data: T, delay: number = 300): Promise<T> => {
  await mockDelay(delay)
  return data
}
