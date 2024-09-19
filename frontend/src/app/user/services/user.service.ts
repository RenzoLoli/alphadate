import { Injectable } from '@angular/core';
import { BaseService } from '../../shared/services/base.service';
import { HttpClient } from '@angular/common/http';
import { retry } from 'rxjs';
import { UserUpdateRequest } from '../models/user-update.request';
import { UserEntity } from '../models/user.entity';

@Injectable({
  providedIn: 'root',
})
export class UserService extends BaseService {
  constructor(http: HttpClient) {
    super(http);
    this.resourceEndPoint = '/user';
  }

  getUserById(id: string) {
    const path = `${this.resourcePath()}/${id}`;
    return this.http.get<UserEntity>(path).pipe(retry(2));
  }

  updateUser(id: string, user: UserUpdateRequest) {
    const path = `${this.resourcePath()}/${id}`;
    return this.http.put<null>(path, user).pipe(retry(2));
  }

  deleteUser(id: string) {
    const path = `${this.resourcePath()}/${id}`;
    return this.http.delete<null>(path).pipe(retry(2));
  }
}
