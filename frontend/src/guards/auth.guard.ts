import userStorage from "@/store/auth.store";

type Next = string | null;

const authGuard = (from: string): Next => {
  const authUrls = ["/login", "/register"];

  const isAuth = userStorage.isAuth();
  const isAuthUrl = authUrls.includes(from);
  if (!isAuthUrl && !isAuth) {
    return "/login";
  }

  if (isAuthUrl && isAuth) {
    return "/";
  }

  return null;
};

export default authGuard;
