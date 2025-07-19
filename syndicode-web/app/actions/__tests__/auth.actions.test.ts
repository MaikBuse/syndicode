import { describe, it, expect, vi, beforeEach } from 'vitest'
import { VerificationCodeExpired, VerificationCodeFalse, UnknownAuthError } from '@/domain/auth/auth.error'

// Mock the auth service
vi.mock('@/application/auth-service', () => ({
  default: {
    verifyUser: vi.fn(),
  },
}))

// Mock the utils
vi.mock('../utils', () => ({
  getClientIp: vi.fn(() => '127.0.0.1'),
}))

// Import after mocking
import { verifyUserAction } from '../auth.actions'
import authService from '@/application/auth-service'

const mockAuthService = vi.mocked(authService)

describe('verifyUserAction error handling', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should return specific error message for expired verification code', async () => {
    // Arrange
    mockAuthService.verifyUser.mockRejectedValue(new VerificationCodeExpired())
    
    const validInput = {
      userName: 'testuser',
      code: '1234567890',
    }

    // Act
    const result = await verifyUserAction(validInput)

    // Assert
    expect(result).toEqual({
      success: false,
      message: 'Your verification code has expired. Please request a new one.',
    })
    expect(mockAuthService.verifyUser).toHaveBeenCalledWith(validInput, '127.0.0.1')
  })

  it('should return specific error message for incorrect verification code', async () => {
    // Arrange
    mockAuthService.verifyUser.mockRejectedValue(new VerificationCodeFalse())
    
    const validInput = {
      userName: 'testuser',
      code: 'wrongcode1',
    }

    // Act
    const result = await verifyUserAction(validInput)

    // Assert
    expect(result).toEqual({
      success: false,
      message: 'The verification code is incorrect. Please check and try again.',
    })
    expect(mockAuthService.verifyUser).toHaveBeenCalledWith(validInput, '127.0.0.1')
  })

  it('should return generic error message for unknown errors', async () => {
    // Arrange
    mockAuthService.verifyUser.mockRejectedValue(new UnknownAuthError('Server error'))
    
    const validInput = {
      userName: 'testuser',
      code: '1234567890',
    }

    // Act
    const result = await verifyUserAction(validInput)

    // Assert
    expect(result).toEqual({
      success: false,
      message: 'Verification failed. Please check the code and try again.',
    })
    expect(mockAuthService.verifyUser).toHaveBeenCalledWith(validInput, '127.0.0.1')
  })

  it('should return generic error message for unexpected errors', async () => {
    // Arrange
    mockAuthService.verifyUser.mockRejectedValue(new Error('Network error'))
    
    const validInput = {
      userName: 'testuser',
      code: '1234567890',
    }

    // Act
    const result = await verifyUserAction(validInput)

    // Assert
    expect(result).toEqual({
      success: false,
      message: 'Verification failed. Please check the code and try again.',
    })
  })

  it('should return success message when verification succeeds', async () => {
    // Arrange
    mockAuthService.verifyUser.mockResolvedValue({ userUuid: 'user-uuid-123' })
    
    const validInput = {
      userName: 'testuser',
      code: '1234567890',
    }

    // Act
    const result = await verifyUserAction(validInput)

    // Assert
    expect(result).toEqual({
      success: true,
      message: 'Verification successful! You can now log in.',
    })
    expect(mockAuthService.verifyUser).toHaveBeenCalledWith(validInput, '127.0.0.1')
  })

  it('should return validation error for invalid input', async () => {
    // Arrange
    const invalidInput = {
      userName: 'ab', // Too short
      code: '123', // Too short
    }

    // Act
    const result = await verifyUserAction(invalidInput)

    // Assert
    expect(result).toEqual({
      success: false,
      message: 'Invalid input.',
    })
    expect(mockAuthService.verifyUser).not.toHaveBeenCalled()
  })

  it('should return validation error for missing userName', async () => {
    // Arrange
    const invalidInput = {
      userName: '',
      code: '1234567890',
    }

    // Act
    const result = await verifyUserAction(invalidInput)

    // Assert
    expect(result).toEqual({
      success: false,
      message: 'Invalid input.',
    })
    expect(mockAuthService.verifyUser).not.toHaveBeenCalled()
  })

  it('should return validation error for missing code', async () => {
    // Arrange
    const invalidInput = {
      userName: 'testuser',
      code: '',
    }

    // Act
    const result = await verifyUserAction(invalidInput)

    // Assert
    expect(result).toEqual({
      success: false,
      message: 'Invalid input.',
    })
    expect(mockAuthService.verifyUser).not.toHaveBeenCalled()
  })
})