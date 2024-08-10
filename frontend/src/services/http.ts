import responseInterceptor from "@/interceptors/response.interceptor";
import tokenInterceptor from "@/interceptors/token.interceptor";
import axios from "axios";

const BACKEND_URL = import.meta.env.BACKEND_URL;

const http = axios.create({
  baseURL: BACKEND_URL,
  headers: {
    "Content-Type": "application/json",
  },
});

http.interceptors.request.use(tokenInterceptor.fulfilled);

http.interceptors.response.use(
  responseInterceptor.fulfilled,
  responseInterceptor.reject,
);

export default http;
