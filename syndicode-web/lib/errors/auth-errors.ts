export class TokenExpiredError extends Error {
  constructor(message: string = 'Token has expired') {
    super(message);
    this.name = 'TokenExpiredError';
  }
}

export class TokenInvalidError extends Error {
  constructor(message: string = 'Token is invalid') {
    super(message);
    this.name = 'TokenInvalidError';
  }
}

export function isAuthError(error: unknown): error is TokenExpiredError | TokenInvalidError {
  return error instanceof TokenExpiredError || error instanceof TokenInvalidError;
}