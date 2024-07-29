import { User } from "@/model/user.entity";
import type { UserRequest } from "@/model/user_request";

export class UserAdapter {
  static requestToEntity(userReq: UserRequest): User {
    let user = new User();
    user.id = userReq.id;
    user.photo = userReq.photo;
    user.userName = userReq.user_name;
    user.coupleName = userReq.couple_name;
    user.email = userReq.email;
    user.anniversary = userReq.anniversary;
    return user;
  }
}
