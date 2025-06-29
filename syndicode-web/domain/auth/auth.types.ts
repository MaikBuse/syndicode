export type UserRegistration = {
  userName: string;
  userPassword: string;
  email: string;
  corporationName: string;
};

export type UserCredentials = {
  userName: string;
  userPassword: string;
};

export type VerificationInfo = {
  userName: string;
  code: string;
};
