import { computed, inject } from '@angular/core';
import { Router } from '@angular/router';
import {
  patchState,
  signalStore,
  withComputed,
  withHooks,
  withMethods,
  withState,
  WritableStateSource,
} from '@ngrx/signals';
import { rxMethod } from '@ngrx/signals/rxjs-interop';
import { catchError, pipe, switchMap, tap, throwError } from 'rxjs';
import { SignInRequest } from '../models/sign-in.request';
import { SignUpRequest } from '../models/sign-up.request';
import {
  checkUnnecessaryUserUpdateRequest,
  UserUpdateRequest,
} from '../models/user-update.request';
import { UserEntity } from '../models/user.entity';
import { AuthService } from '../services/auth.service';
import AuthLocalStorageService from '../services/user-local-storage.service';
import { UserService } from '../services/user.service';
import { TokenUtils } from '../utils/token.utils';
import { TokenService } from '../services/token.service';

export interface AuthState {
  token: string | null;
  user: UserEntity | null;
  loading: boolean;
  error: string | null;
  authenticated: boolean;
}

const initialState: AuthState = {
  token: null,
  user: null,
  error: null,
  loading: false,
  authenticated: false,
};

const clearState = (store: WritableStateSource<any>) => {
  patchState(store, {
    token: null,
    user: null,
    error: null,
    loading: false,
    authenticated: false,
  });
};

const errorState = (store: WritableStateSource<any>, error: any) => {
  let message = 'An error occurred';
  if (typeof error === 'string') {
    message = error;
  } else if (typeof error === 'object' && error.message) {
    message = error.message;
  }

  patchState(store, {
    token: null,
    user: null,
    error: message,
    loading: false,
    authenticated: false,
  });
};

export const AuthStore = signalStore(
  { providedIn: 'root' },
  withState(initialState),
  withComputed((state) => ({
    isAuthenticated: computed(() => state.authenticated()),
    isLoading: computed(() => state.loading()),
    getError: computed(() => state.error()),
    getToken: computed(() => state.token()),
    getUser: computed(() => state.user()),
  })),
  withMethods(
    (
      store,
      router = inject(Router),
      authService = inject(AuthService),
      userService = inject(UserService),
      tokenService = inject(TokenService),
      authLocalStorageService = inject(AuthLocalStorageService),
    ) => ({
      logout: () => {
        clearState(store);
        authLocalStorageService.clear();
      },

      login: rxMethod<SignInRequest>(
        pipe(
          tap(() => {
            patchState(store, { loading: true });
          }),
          switchMap((signinRequest) => {
            return authService.login(signinRequest);
          }),
          switchMap((response) => {
            const resToken = response.token;
            if (!resToken) return throwError(() => new Error('Invalid token'));

            const userId = TokenUtils.getUserIdFromToken(resToken);
            if (!userId) return throwError(() => new Error('Invalid token'));

            authLocalStorageService.setToken(resToken);

            patchState(store, { token: resToken });

            return userService.getUserById(userId);
          }),
          tap((user) => {
            patchState(store, {
              authenticated: true,
              user,
              loading: false,
              error: null,
            });

            authLocalStorageService.setUser(user);

            router.navigate(['/']);
          }),
          catchError((error, caught) => {
            errorState(store, error);

            authLocalStorageService.clear();
            return caught;
          }),
        ),
      ),

      register: rxMethod<SignUpRequest>(
        pipe(
          tap(() => patchState(store, { loading: true })),
          switchMap((signupRequest) => {
            return authService.register(signupRequest);
          }),
          tap(() => {
            patchState(store, { loading: false });
            router.navigate(['/login']);
          }),
          catchError((error, caught) => {
            errorState(store, error);
            return caught;
          }),
        ),
      ),

      updateUser: rxMethod<UserUpdateRequest>(
        pipe(
          tap(() => patchState(store, { loading: true })),
          switchMap((userUpdateRequest) => {
            if (!checkUnnecessaryUserUpdateRequest(userUpdateRequest))
              return throwError(() => new Error('Invalid user update request'));

            const user = store.getUser();
            if (!user) return throwError(() => new Error('Invalid user'));

            return userService.updateUser(user.id, userUpdateRequest);
          }),
          switchMap(() => {
            const user = store.getUser();
            if (!user) return throwError(() => new Error('Invalid user'));

            return userService.getUserById(user.id);
          }),
          tap((user) => {
            authLocalStorageService.setUser(user);
            patchState(store, {
              loading: false,
              user,
            });
          }),
          catchError((error, caught) => {
            patchState(store, { loading: false, error: error.message });
            return caught;
          }),
        ),
      ),
      renewtoken: rxMethod<void>(
        pipe(
          tap(() => {
            patchState(store, { loading: true });
          }),
          switchMap(() => {
            return tokenService.renew();
          }),
          switchMap((tokenResponse) => {
            authLocalStorageService.setToken(tokenResponse.token);
            patchState(store, {
              token: tokenResponse.token,
              loading: false,
              authenticated: true,
            });

            const userId = TokenUtils.getUserIdFromToken(tokenResponse.token);
            if (!userId) return throwError(() => new Error('Invalid token'));

            return userService.getUserById(userId);
          }),
          tap((user) => {
            authLocalStorageService.setUser(user);
            patchState(store, {
              loading: false,
              user,
            });
          }),
          catchError((_, caught) => {
            clearState(store);
            authLocalStorageService.clear();
            router.navigate(['/login']);
            return caught;
          }),
        ),
      ),
    }),
  ),
  withHooks(
    (state, authLocalStorageService = inject(AuthLocalStorageService)) => ({
      onInit: () => {
        const token = authLocalStorageService.getToken();
        const user = authLocalStorageService.getUser();

        if (token && user) {
          patchState(state, {
            token,
            user,
            authenticated: true,
          });
        }
      },
      onDestroy: () => {
        authLocalStorageService.clear();
      },
    }),
  ),
);
