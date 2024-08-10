import { TokenService } from "@/services/token.service";
import authStorage from "@/store/auth.store";

type Next = string | null;

const expTokenGuard = (): Next => {
  const tokenReq = authStorage.getToken();
  if (!tokenReq || !tokenReq.token) {
    return "login";
  }

  const tokenService = new TokenService();
  const exp = tokenService.getExpirationFromToken(tokenReq);
  if (!exp) null;

  return "renew";
};

export default expTokenGuard;
