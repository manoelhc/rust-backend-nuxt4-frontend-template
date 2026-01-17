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
      group_id: '660e8400-e29b-41d4-a716-446655440001',
      properties: {
        department: 'Engineering',
        role: 'Developer'
      },
      created_at: '2026-01-01T00:00:00Z',
      updated_at: '2026-01-16T00:00:00Z'
    }
  },
  
  // Admin endpoints mock data
  roles: [
    {
      role: {
        id: '770e8400-e29b-41d4-a716-446655440010',
        name: 'Admin',
        description: 'Full system administrator with all permissions',
        is_admin: true,
        created_at: '2026-01-01T00:00:00Z',
        updated_at: '2026-01-01T00:00:00Z'
      },
      permissions: [
        {
          id: '880e8400-e29b-41d4-a716-446655440020',
          role_id: '770e8400-e29b-41d4-a716-446655440010',
          page: 'dashboard',
          can_view: true,
          can_edit: true,
          can_view_own: true,
          can_edit_own: true,
          can_view_ours: true,
          can_edit_ours: true,
          created_at: '2026-01-01T00:00:00Z',
          updated_at: '2026-01-01T00:00:00Z'
        },
        {
          id: '880e8400-e29b-41d4-a716-446655440021',
          role_id: '770e8400-e29b-41d4-a716-446655440010',
          page: 'users',
          can_view: true,
          can_edit: true,
          can_view_own: true,
          can_edit_own: true,
          can_view_ours: true,
          can_edit_ours: true,
          created_at: '2026-01-01T00:00:00Z',
          updated_at: '2026-01-01T00:00:00Z'
        },
        {
          id: '880e8400-e29b-41d4-a716-446655440022',
          role_id: '770e8400-e29b-41d4-a716-446655440010',
          page: 'roles',
          can_view: true,
          can_edit: true,
          can_view_own: true,
          can_edit_own: true,
          can_view_ours: true,
          can_edit_ours: true,
          created_at: '2026-01-01T00:00:00Z',
          updated_at: '2026-01-01T00:00:00Z'
        }
      ]
    },
    {
      role: {
        id: '770e8400-e29b-41d4-a716-446655440011',
        name: 'View',
        description: 'View-only access to assigned pages',
        is_admin: false,
        created_at: '2026-01-01T00:00:00Z',
        updated_at: '2026-01-01T00:00:00Z'
      },
      permissions: [
        {
          id: '880e8400-e29b-41d4-a716-446655440030',
          role_id: '770e8400-e29b-41d4-a716-446655440011',
          page: 'dashboard',
          can_view: true,
          can_edit: false,
          can_view_own: false,
          can_edit_own: false,
          can_view_ours: false,
          can_edit_ours: false,
          created_at: '2026-01-01T00:00:00Z',
          updated_at: '2026-01-01T00:00:00Z'
        }
      ]
    }
  ],
  
  users: [
    {
      user: {
        id: '550e8400-e29b-41d4-a716-446655440000',
        sub: 'user123',
        user_email: 'john.doe@example.com',
        user_fullname: 'John Doe',
        organization: 'Acme Corp',
        group_id: '660e8400-e29b-41d4-a716-446655440001',
        properties: {},
        created_at: '2026-01-01T00:00:00Z',
        updated_at: '2026-01-16T00:00:00Z'
      },
      roles: [
        {
          id: '770e8400-e29b-41d4-a716-446655440010',
          name: 'Admin',
          description: 'Full system administrator with all permissions',
          is_admin: true,
          created_at: '2026-01-01T00:00:00Z',
          updated_at: '2026-01-01T00:00:00Z'
        }
      ]
    },
    {
      user: {
        id: '550e8400-e29b-41d4-a716-446655440001',
        sub: 'user456',
        user_email: 'jane.smith@example.com',
        user_fullname: 'Jane Smith',
        organization: 'Acme Corp',
        group_id: '660e8400-e29b-41d4-a716-446655440001',
        properties: {},
        created_at: '2026-01-02T00:00:00Z',
        updated_at: '2026-01-16T00:00:00Z'
      },
      roles: [
        {
          id: '770e8400-e29b-41d4-a716-446655440011',
          name: 'View',
          description: 'View-only access to assigned pages',
          is_admin: false,
          created_at: '2026-01-01T00:00:00Z',
          updated_at: '2026-01-01T00:00:00Z'
        }
      ]
    },
    {
      user: {
        id: '550e8400-e29b-41d4-a716-446655440002',
        sub: 'user789',
        user_email: 'bob.wilson@example.com',
        user_fullname: 'Bob Wilson',
        organization: 'Acme Corp',
        group_id: '660e8400-e29b-41d4-a716-446655440002',
        properties: {},
        created_at: '2026-01-03T00:00:00Z',
        updated_at: '2026-01-16T00:00:00Z'
      },
      roles: []
    }
  ]
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
