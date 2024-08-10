import authStorage from "@/store/auth.store";
import axios from "axios";

const BACKEND_URL = import.meta.env.BACKEND_URL;

const http = axios.create({
  baseURL: BACKEND_URL,
  headers: {
    "Content-Type": "application/json",
  },
});

http.interceptors.request.use((req) => {
  const token = authStorage.getToken()?.token;
  const authorization = "Bearer " + token;
  req.headers.Authorization = authorization;
  return req;
});

http.interceptors.response.use(
  (res) => res,
  (error) => {
    let message = error;

    if (error.response && error.response.data) {
      message = error.response.data.message;
    }

    console.log(`intercept axios error:`, message);
    return Promise.reject(message);
  },
);

export default http;
