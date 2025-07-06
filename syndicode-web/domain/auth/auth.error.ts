export class AuthError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'AuthError';
  }
}

export class UserInactiveError extends AuthError {
  constructor(message = 'User account is inactive and requires verification.') {
    super(message);
    this.name = 'UserInactiveError';
  }
}

export class InvalidCredentialsError extends AuthError {
  constructor(message = 'The username or password provided is incorrect.') {
    super(message);
    this.name = 'InvalidCredentialsError';
  }
}

export class UnauthenticatedError extends AuthError {
  constructor(message = 'User is not authenticated.') {
    super(message);
    this.name = 'UnauthenticatedError';
  }
}

export class VerificationCodeExpired extends AuthError {
  constructor(message = 'The verification code has expired.') {
    super(message);
    this.name = 'VerificationCodeExpired';
  }
}

export class VerificationCodeFalse extends AuthError {
  constructor(message = 'The provided verification is false.') {
    super(message);
    this.name = 'VerificationCodeFalse';
  }
}

export class UniqueConstraint extends AuthError {
  constructor(message = "A unique constraint was violated.") {
    super(message);
    this.name = 'UniqueConstraint';
  }

  toString(): string {
    return this.message;
  }
}

export class UnknownAuthError extends AuthError {
  constructor(message = 'An unexpected authentication error occurred.') {
    super(message);
    this.name = 'UnknownAuthError';
  }
}
