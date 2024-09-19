import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { BaseService } from '../../shared/services/base.service';
import { SignInRequest } from '../models/sign-in.request';
import { SignInResponse } from '../models/sign-in.response';
import { SignUpRequest } from '../models/sign-up.request';
import { retry } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class AuthService extends BaseService {
  constructor(http: HttpClient) {
    super(http);
    this.resourceEndPoint = '/auth';
  }

  login(signinRequest: SignInRequest) {
    const path = `${this.resourcePath()}/login`;
    return this.http.post<SignInResponse>(path, signinRequest).pipe(retry(2));
  }

  register(signUpRequest: SignUpRequest) {
    const path = `${this.resourcePath()}/register`;
    return this.http.post<null>(path, signUpRequest).pipe(retry(2));
  }

  logout() {
    const path = `${this.resourcePath()}/logout`;
    return this.http.post<null>(path, null).pipe(retry(2));
  }
}
