export interface UserUpdateRequest {
  username?: string;
  couplename?: string;
  anniversary?: string;
  photo?: string;
}

export const checkUnnecessaryUserUpdateRequest = (
  userRequest: UserUpdateRequest,
) => {
  return (
    Boolean(userRequest.username) &&
    Boolean(userRequest.couplename) &&
    Boolean(userRequest.anniversary) &&
    Boolean(userRequest.photo)
  );
};
