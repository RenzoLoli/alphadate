import userStorage from "@/store/auth.store";

type Redirect = (path: string) => Response;

const authGuard = (from: string, redirect: Redirect): Response | null => {
  const authUrls = ["/login", "/register"];

  const isAuth = userStorage.isAuth();
  const isAuthUrl = authUrls.includes(from);
  if (!isAuthUrl && !isAuth) {
    return redirect("/login");
  }

  if(isAuthUrl && isAuth){
    return redirect("/");
  }

  return null;
};

export default authGuard;
