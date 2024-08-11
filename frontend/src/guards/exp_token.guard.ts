import authStorage from "@/store/auth.store";
import { TokenUtils } from "@/utils/token.utils";

type Next = string | null;

const expTokenGuard = (from: string): Next => {
  const authUrls = ["/login", "/register"];

  const isAuthUrl = authUrls.includes(from);
  if(isAuthUrl) return null;

  const tokenReq = authStorage.getToken();
  if (!tokenReq) {
    return "login";
  }

  const tokenUtils = new TokenUtils();
  const exp = tokenUtils.isExpiredIn(tokenReq, 60 * 1000);
  if (!exp) return null;

  return "renew";
};

export default expTokenGuard;
