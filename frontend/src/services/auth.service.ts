import type { SignInRequest } from "@/model/signin.request";
import type { SignUpRequest } from "@/model/signup.request";
import type { AxiosResponse } from "axios";
import type { TokenResponse } from "@/model/token_response";
import http from "./http";

type OkResponse = Promise<AxiosResponse<null>>;
type PromiseResponse = Promise<AxiosResponse<TokenResponse>>;

export class AuthService {
  login(signinRequest: SignInRequest): PromiseResponse {
    return http.post<TokenResponse>("/auth/login", signinRequest);
  }

  register(signupRequest: SignUpRequest): OkResponse {
    return http.post("/auth/register", signupRequest);
  }
}
