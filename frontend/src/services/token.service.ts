import type { Token } from "@/model/token";

type IdToken = {
  id: string;
};

export class TokenService {
  getUserIdFromToken(tokenReq: Token): IdToken | null {
    const { token } = tokenReq;
    try {
      const payload = JSON.parse(this.getPayload(token));
      const sub = payload.sub;
      const idToken = JSON.parse(sub) as IdToken;
      return idToken;
    } catch {
      return null;
    }
  }

  private getPayload(token: string): string {
    return atob(token.split(".")[1]);
  }
}
