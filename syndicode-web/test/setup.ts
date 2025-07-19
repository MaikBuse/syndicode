// Test setup file for Vitest
import { vi } from 'vitest'

// Mock Next.js modules
vi.mock('next/headers', () => ({
  headers: vi.fn(() => Promise.resolve({
    get: vi.fn(() => '127.0.0.1'),
  })),
}))

// Mock toast notifications
vi.mock('sonner', () => ({
  toast: {
    success: vi.fn(),
    error: vi.fn(),
  },
}))