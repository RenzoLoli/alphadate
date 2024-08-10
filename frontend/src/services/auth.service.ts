import type { SignInRequest } from "@/model/signin.request";
import type { SignUpRequest } from "@/model/signup.request";
import type { AxiosResponse } from "axios";
import http from "./http";
import type { Token } from "@/model/token";

type OkResponse = Promise<AxiosResponse<null>>;
type PromiseResponse = Promise<AxiosResponse<Token>>;

export class AuthService {
  login(signinRequest: SignInRequest): PromiseResponse {
    return http.post<Token>("/auth/login", signinRequest);
  }

  register(signupRequest: SignUpRequest): OkResponse {
    return http.post("/auth/register", signupRequest);
  }

  renew(id: string): PromiseResponse {
    return http.post<Token>("/auth/renew", {
      id,
    });
  }
}
