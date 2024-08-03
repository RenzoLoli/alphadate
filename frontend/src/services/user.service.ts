import { User } from "@/model/user.entity";
import http from "./http";

export class UserApiService {
  async getUserById(id: string): Promise<User | null> {
    const res = await http.get("/users", {
      params: {
        id,
      },
    });
    return null;
  }
}
