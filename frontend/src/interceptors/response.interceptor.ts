const fulfilled = (config: any) => config;
const reject = (error: any) => {
  let message = error;

  if (error.response && error.response.data) {
    message = error.response.data.message;
  }

  console.log(`intercept axios error:`, message);
  return Promise.reject(message);
};

export default { fulfilled, reject };
