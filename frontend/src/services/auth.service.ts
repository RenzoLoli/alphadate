import type { SignInRequest } from "@/model/signin.request";
import type { SignUpRequest } from "@/model/signup.request";
import type { User } from "@/model/user.entity";
import type { AxiosResponse } from "axios";
import http from "./http";

type PromiseResponse = Promise<AxiosResponse<User, string>>;

export class AuthService {
  login(signinRequest: SignInRequest): PromiseResponse {
    return http.post<User>("/login", signinRequest);
  }

  register(signupRequest: SignUpRequest): PromiseResponse {
    return http.post<User>("/register", signupRequest);
  }

  logout(): PromiseResponse {
    return http.post("/logout");
  }
}
