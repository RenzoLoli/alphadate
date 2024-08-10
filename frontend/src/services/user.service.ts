import http from "./http";
import type { AxiosResponse } from "axios";
import type { GetUserResponse } from "@/model/get_user.response";

type PromiseResponse<T> = Promise<AxiosResponse<T>>;

export class UserApiService {
  getUsers(): PromiseResponse<GetUserResponse[]> {
    return http.get("/auth/users");
  }

  getUserById(id: string): PromiseResponse<GetUserResponse> {
    return http.get("/user?id=" + id);
  }
}
