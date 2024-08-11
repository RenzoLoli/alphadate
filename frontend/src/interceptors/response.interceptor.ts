type ErrorResponseData = {
  message: string;
};

const fulfilled = (config: any) => config;
const reject = (error: any) => {
  let message = error;

  if (error.response && error.response.data) {
    const errorResponseData = error.response.data as ErrorResponseData;
    message = errorResponseData.message;
  }

  console.log(`intercept axios error:`, message);
  return Promise.reject(message);
};

export default { fulfilled, reject };
