import { User } from "@/model/user.entity";
import http from "./http";

export class UserApiService {
  getUsers(): Promise<User[]> {
    return http.get("/auth/users");
  }

  getUserById(id: string): Promise<User> {
    return http.get("/user", {
      params: {
        id,
      },
    });
  }
}
