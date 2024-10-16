const FRONTEND_API_URL = process.env['FRONTEND_API_URL'];

export const environment = {
  production: true,
  apiUrl: FRONTEND_API_URL || 'http://127.0.0.1:3000/api/v1',
};
