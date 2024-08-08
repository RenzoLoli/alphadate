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
    let message = error;

    if (error.response && error.response.data) {
      message = error.response.data;
    } else {
      message = error.message;
    }

    console.log(`intercept axios error:`, message);
    return Promise.reject(message);
  },
);

export default http;
