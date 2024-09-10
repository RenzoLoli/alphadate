export interface UserEntity {
  id: string;
  email: string;
  // TODO: remove password from user entity
  password: string;
  username: string;
  couplename: string;
  photo: string;
  anniversary: string;
}
