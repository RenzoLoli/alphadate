import type { GetUserResponse } from "@/model/get_user.response";
import { User } from "@/model/user.entity";

export class UserAdapter {
  entityFromGetUserResponse(getUserResponse: GetUserResponse): User {
    return new User(
      getUserResponse.id,
      getUserResponse.username,
      getUserResponse.couplename,
      getUserResponse.photo, 
      getUserResponse.email,
      new Date(getUserResponse.anniversary)
    );
  }
}
