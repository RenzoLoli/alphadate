import { inject, Injectable } from '@angular/core';
import {
  ActivatedRouteSnapshot,
  CanActivate,
  GuardResult,
  MaybeAsync,
  Router,
  RouterStateSnapshot,
} from '@angular/router';
import { AuthStore } from '../store/auth.store';

@Injectable({
  providedIn: 'root',
})
export default class AuthGuard implements CanActivate {
  AuthStore = inject(AuthStore);
  constructor(private router: Router) {}
  canActivate(
    _route: ActivatedRouteSnapshot,
    state: RouterStateSnapshot,
  ): MaybeAsync<GuardResult> {
    const isAuthenticated = this.AuthStore.isAuthenticated();
    const currentPath = state.url;
    const isAuthUrl = ['/login', '/register'].includes(currentPath);

    if (isAuthenticated && isAuthUrl) {
      return this.router.parseUrl('/');
    }

    if (!isAuthenticated && !isAuthUrl) {
      return this.router.parseUrl('/login');
    }

    return true;
  }
}
