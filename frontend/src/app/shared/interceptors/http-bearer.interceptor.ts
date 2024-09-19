import { HttpInterceptorFn } from '@angular/common/http';
import { inject } from '@angular/core';
import { AuthStore } from '../../user/store/auth.store';
import { TokenUtils } from '../../user/utils/token.utils';

export const httpBearerInterceptor: HttpInterceptorFn = (req, next) => {
  const authStore = inject(AuthStore);

  const token = authStore.getToken();
  if (!token) return next(req);

  const twoHours = 1000 * 60 * 60 * 2;
  if (TokenUtils.isExpiredIn(token, twoHours)) {
    authStore.renewtoken();
  }

  const nReq = req.clone({
    headers: req.headers.set('Authorization', `Bearer ${authStore.token()}`),
  });

  return next(nReq);
};
