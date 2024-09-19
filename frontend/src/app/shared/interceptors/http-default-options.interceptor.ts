import { HttpInterceptorFn } from '@angular/common/http';

export const httpDefaultOptionsInterceptor: HttpInterceptorFn = (req, next) => {
  const nReq = req.clone({
    headers: req.headers
      .set('Content-Type', 'application/json')
      .set('Accept', 'application/json'),
    withCredentials: true,
  });
  return next(nReq);
};
