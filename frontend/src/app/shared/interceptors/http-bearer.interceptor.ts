import { HttpInterceptorFn } from '@angular/common/http';
import { inject } from '@angular/core';
import { AuthStore } from '../../user/store/auth.store';
import { TokenUtils } from '../../user/utils/token.utils';
import { Router } from '@angular/router';

export const httpBearerInterceptor: HttpInterceptorFn = (req, next) => {
  const authStore = inject(AuthStore);
  const router = inject(Router);

  const token = authStore.getToken();
  if (!token) return next(req);

  // const twoHours = 1000 * 60 * 60 * 2;
  // if (TokenUtils.isExpiredIn(token, twoHours)) {
  //   authStore.renewtoken();
  //   return next(req);
  // }
  //
  if (TokenUtils.isExpired(token)) {
    router.navigate(['/logout']);
    return next(req);
  }

  const nReq = req.clone({
    headers: req.headers.set('Authorization', `Bearer ${authStore.token()}`),
  });

  return next(nReq);
};
