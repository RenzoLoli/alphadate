import axios from "axios";

const http = axios.create({
  baseURL: "http://google.com",
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
