import { describe, it, expect, vi, beforeEach } from 'vitest'
import { cookies } from 'next/headers'
import { jwtVerify } from 'jose'
import { getCurrentUser, clearExpiredAuthToken } from '../auth'

// Mock next/headers
vi.mock('next/headers', () => ({
  cookies: vi.fn(),
}))

// Mock jose
vi.mock('jose', () => ({
  jwtVerify: vi.fn(),
}))

// Mock server config
vi.mock('@/config/server', () => ({
  serverConfig: {
    jwt_secret: 'test-secret',
  },
}))

describe('getCurrentUser', () => {
  const mockCookieStore = {
    get: vi.fn(),
    delete: vi.fn(),
  }

  beforeEach(() => {
    vi.clearAllMocks()
    vi.mocked(cookies).mockResolvedValue(mockCookieStore as ReturnType<typeof cookies>)
  })

  it('should return null when no auth token exists', async () => {
    mockCookieStore.get.mockReturnValue(undefined)

    const result = await getCurrentUser()

    expect(result).toBeNull()
    expect(mockCookieStore.get).toHaveBeenCalledWith('auth_token')
  })

  it('should return user when valid JWT exists', async () => {
    const mockJwtPayload = {
      sub: 'user-123',
      user_name: 'testuser',
      user_email: 'test@example.com',
      user_role: 'user',
    }

    mockCookieStore.get.mockReturnValue({ value: 'valid-jwt-token' })
    vi.mocked(jwtVerify).mockResolvedValue({ 
      payload: mockJwtPayload,
      protectedHeader: { alg: 'HS256' }
    })

    const result = await getCurrentUser()

    expect(result).toEqual({
      uuid: 'user-123',
      name: 'testuser',
      email: 'test@example.com',
      role: 'user',
    })
  })

  it('should return null when JWT verification fails (expired token)', async () => {
    mockCookieStore.get.mockReturnValue({ value: 'expired-jwt-token' })
    vi.mocked(jwtVerify).mockRejectedValue(new Error('JWT expired'))

    const result = await getCurrentUser()

    expect(result).toBeNull()
    expect(mockCookieStore.delete).not.toHaveBeenCalled() // Should not delete cookie in Server Component
  })

  it('should log error when JWT verification fails', async () => {
    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {})
    mockCookieStore.get.mockReturnValue({ value: 'invalid-jwt-token' })
    vi.mocked(jwtVerify).mockRejectedValue(new Error('Invalid JWT'))

    await getCurrentUser()

    expect(consoleSpy).toHaveBeenCalledWith('JWT Verification Error:', expect.any(Error))
    consoleSpy.mockRestore()
  })
})

describe('clearExpiredAuthToken', () => {
  const mockCookieStore = {
    delete: vi.fn(),
  }

  beforeEach(() => {
    vi.clearAllMocks()
    vi.mocked(cookies).mockResolvedValue(mockCookieStore as ReturnType<typeof cookies>)
  })

  it('should delete auth_token cookie', async () => {
    await clearExpiredAuthToken()

    expect(mockCookieStore.delete).toHaveBeenCalledWith('auth_token')
  })
})