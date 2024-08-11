import type { TokenResponse } from "@/model/token_response";
import type { AxiosResponse } from "axios";
import http from "./http";

type PromiseResponse = Promise<AxiosResponse<TokenResponse>>;

export class TokenService {
  renew(): PromiseResponse {
    return http.post<TokenResponse>("/token/renew");
  }
}
