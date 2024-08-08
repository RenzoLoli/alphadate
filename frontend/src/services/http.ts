import axios from "axios";

const BACKEND_URL = import.meta.env.BACKEND_URL;

const http = axios.create({
  baseURL: BACKEND_URL,
  headers: {
    "Content-Type": "json",
  },
});

http.interceptors.response.use(
  (res) => res,
  (error) => {
    console.log(`intercept axios error:`, error.message);
    return Promise.reject(error.message);
  },
);

export default http;
