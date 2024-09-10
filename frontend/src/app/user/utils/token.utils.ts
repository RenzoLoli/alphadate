interface Payload {
  sub: string;
  exp: number;
}

export class TokenUtils {
  static getUserIdFromToken(token: string): string | null {
    const payload = TokenUtils.getPayload(token);
    if (!payload) return null;

    const { sub } = payload;
    return sub;
  }

  static isExpiredIn(token: string, time: number): boolean {
    const now = Date.now();
    const expirationTime = this.getExpirationFromToken(token);
    if (!expirationTime) return true;

    const diff = expirationTime.getTime() - now;

    return diff > 0 && diff <= time;
  }

  private static getExpirationFromToken(token: string): Date | null {
    const payload = this.getPayload(token);
    if (!payload) return null;

    const { exp } = payload;
    const expDate = new Date(exp);
    return expDate;
  }

  private static getPayload(token: string): Payload | null {
    try {
      const payloadStr = atob(token.split('.')[1]);
      return JSON.parse(payloadStr) as Payload;
    } catch {
      return null;
    }
  }
}
