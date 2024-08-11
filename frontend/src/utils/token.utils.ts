import type { Token } from "@/model/token_type";

type IdToken = string;

type Payload = {
  sub: string;
  exp: number;
};

export class TokenUtils {
  getUserIdFromToken(token: Token): IdToken | null {
    const payload = this.getPayload(token);
    if (!payload) return null;

    const { sub } = payload;
    return sub;
  }

  private getExpirationFromToken(token: Token): Date | null {
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
