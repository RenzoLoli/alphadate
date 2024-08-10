import type { Token } from "@/model/token";

type IdToken = {
  id: string;
};

type Payload = {
  sub: string;
  exp: number;
};

export class TokenService {
  getUserIdFromToken(tokenReq: Token): IdToken | null {
    const { token } = tokenReq;

    const payload = this.getPayload(token);
    if (!payload) return null;

    const { sub } = payload;
    const idToken = JSON.parse(sub) as IdToken;
    return idToken;
  }

  getExpirationFromToken(tokenReq: Token): Date | null {
    const { token } = tokenReq;

    const payload = this.getPayload(token);
    if (!payload) return null;

    const { exp } = payload;
    const expDate = new Date(exp);
    return expDate;
  }

  isExpiredIn(token: Token, time: number): boolean {
    const now = Date.now();
    const expirationTime = this.getExpirationFromToken(token);
    if (!expirationTime) return true;

    const diff = expirationTime.getTime() - now;

    return diff > 0 && diff <= time;
  }

  private getPayload(token: string): Payload | null {
    try {
      const payloadStr = atob(token.split(".")[1]);
      return JSON.parse(payloadStr) as Payload;
    } catch {
      return null;
    }
  }
}
