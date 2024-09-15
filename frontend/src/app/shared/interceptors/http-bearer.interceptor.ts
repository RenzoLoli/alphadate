import { HttpInterceptorFn } from '@angular/common/http';
import { inject } from '@angular/core';
import { AuthStore } from '../../user/store/auth.store';

export const httpBearerInterceptor: HttpInterceptorFn = (req, next) => {
  const authStore = inject(AuthStore);
  if (!authStore.getToken()) return next(req);

  const nReq = req.clone({
    headers: req.headers.set('Authorization', `Bearer ${authStore.token()}`),
  });

  return next(nReq);
};
