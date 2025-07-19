import { describe, it, expect, vi, beforeEach } from 'vitest'
import * as grpc from '@grpc/grpc-js'
import { GrpcAuthRepository } from '../grpc-auth-repository'
import { VerificationCodeExpired, VerificationCodeFalse, UnknownAuthError } from '@/domain/auth/auth.error'

// Mock the gRPC client
const mockVerifyUser = vi.fn()
const mockClient = {
  verifyUser: mockVerifyUser,
}

// Mock the auth client factory
vi.mock('@/lib/grpc/auth-client', () => ({
  getAuthServiceClient: vi.fn(() => mockClient),
}))

describe('GrpcAuthRepository - verifyUser error handling', () => {
  let repository: GrpcAuthRepository
  
  beforeEach(() => {
    repository = new GrpcAuthRepository()
    vi.clearAllMocks()
  })

  it('should throw VerificationCodeExpired when gRPC returns DEADLINE_EXCEEDED', async () => {
    // Arrange
    const mockError = {
      code: grpc.status.DEADLINE_EXCEEDED,
      message: 'Deadline exceeded',
      details: 'The verification code has expired',
    }

    mockVerifyUser.mockImplementation((request, metadata, options, callback) => {
      callback(mockError, null)
    })

    const verificationData = {
      userName: 'testuser',
      code: '1234567890',
    }

    // Act & Assert
    await expect(
      repository.verifyUser(verificationData, '127.0.0.1')
    ).rejects.toThrow(VerificationCodeExpired)
  })

  it('should throw VerificationCodeFalse when gRPC returns INVALID_ARGUMENT', async () => {
    // Arrange
    const mockError = {
      code: grpc.status.INVALID_ARGUMENT,
      message: 'Invalid argument',
      details: 'The provided verification code is false',
    }

    mockVerifyUser.mockImplementation((request, metadata, options, callback) => {
      callback(mockError, null)
    })

    const verificationData = {
      userName: 'testuser',
      code: 'wrongcode1',
    }

    // Act & Assert
    await expect(
      repository.verifyUser(verificationData, '127.0.0.1')
    ).rejects.toThrow(VerificationCodeFalse)
  })

  it('should throw UnknownAuthError for unexpected gRPC error codes', async () => {
    // Arrange
    const mockError = {
      code: grpc.status.INTERNAL,
      message: 'Internal error',
      details: 'Unexpected server error',
    }

    mockVerifyUser.mockImplementation((request, metadata, options, callback) => {
      callback(mockError, null)
    })

    const verificationData = {
      userName: 'testuser',
      code: '1234567890',
    }

    // Act & Assert
    await expect(
      repository.verifyUser(verificationData, '127.0.0.1')
    ).rejects.toThrow(UnknownAuthError)
  })

  it('should resolve successfully when verification succeeds', async () => {
    // Arrange
    const mockResponse = {
      getUserUuid: vi.fn(() => 'user-uuid-123'),
    }

    mockVerifyUser.mockImplementation((request, metadata, options, callback) => {
      callback(null, mockResponse)
    })

    const verificationData = {
      userName: 'testuser',
      code: '1234567890',
    }

    // Act
    const result = await repository.verifyUser(verificationData, '127.0.0.1')

    // Assert
    expect(result).toEqual({ userUuid: 'user-uuid-123' })
    expect(mockResponse.getUserUuid).toHaveBeenCalled()
  })

  it('should set correct request parameters when calling gRPC', async () => {
    // Arrange
    const mockResponse = {
      getUserUuid: vi.fn(() => 'user-uuid-123'),
    }

    let capturedRequest: any
    mockVerifyUser.mockImplementation((request, metadata, options, callback) => {
      capturedRequest = request
      callback(null, mockResponse)
    })

    const verificationData = {
      userName: 'testuser',
      code: '1234567890',
    }

    // Act
    await repository.verifyUser(verificationData, '127.0.0.1')

    // Assert
    expect(mockVerifyUser).toHaveBeenCalledWith(
      expect.any(Object), // request object
      expect.any(Object), // metadata
      expect.objectContaining({
        customContext: expect.objectContaining({
          ipAddress: '127.0.0.1',
        }),
      }), // options
      expect.any(Function) // callback
    )
  })
})