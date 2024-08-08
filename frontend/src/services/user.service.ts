import { User } from "@/model/user.entity";
import http from "./http";
import type { AxiosResponse } from "axios";

type PromiseResponse<T> = Promise<AxiosResponse<T>>;

export class UserApiService {
  getUsers(): PromiseResponse<User[]> {
    return http.get("/auth/users");
  }

  getUserById(id: string): PromiseResponse<User> {
    return http.get("/user?id=" + id);
  }
}
