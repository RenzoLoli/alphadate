import userStorage from "@/store/auth.store";

type Redirect = (path: string) => Response;

const authGuard = (from: string, redirect: Redirect): Response | null => {
  const authUrls = ["/login", "/register"];

  const user = userStorage.getUser();
  const isAuthUrl = authUrls.includes(from);
  if (!isAuthUrl && !user) {
    return redirect("/login");
  }

  if(isAuthUrl && user){
    return redirect("/");
  }

  return null;
};

export default authGuard;
