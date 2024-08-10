import authStorage from "@/store/auth.store";

const fulfilled = (req: any) => {
  if (!authStorage.isAuth()) return req;

  const token = authStorage.getToken()?.token;
  const authorization = "Bearer " + token;
  req.headers.Authorization = authorization;
  return req;
};

export default { fulfilled };