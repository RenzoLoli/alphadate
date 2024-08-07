import type { Token } from "@/model/token";

type IdToken = {
  id: string;
};

export class TokenService {
  getUserIdFromToken(tokenReq: Token): IdToken | null {
    const { token } = tokenReq;
    try {
      return JSON.parse(this.getPayload(token)) as IdToken;
    } catch {
      return null;
    }
  }

  private getPayload(token: string): string {
    return atob(token.split(".")[1]);
  }
}
