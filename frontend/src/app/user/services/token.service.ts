import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable, retry } from 'rxjs';
import { BaseService } from '../../shared/services/base.service';
import { TokenResponse } from '../models/token.response';

@Injectable({
  providedIn: 'root',
})
export class TokenService extends BaseService {
  constructor(http: HttpClient) {
    super(http);
    this.resourceEndPoint = '/token';
  }

  renew(): Observable<TokenResponse> {
    const path = `${this.resourcePath()}/renew`;
    return this.http.get<TokenResponse>(path).pipe(retry(2));
  }
}
