import { Injectable } from '@angular/core';
import { UserEntity } from '../models/user.entity';

@Injectable({
  providedIn: 'root',
})
export default class UserLocalStorageService {
  getToken() {
    return localStorage.getItem('token');
  }

  setToken(token: string) {
    localStorage.setItem('token', token);
  }

  removeToken() {
    localStorage.removeItem('token');
  }

  getUser() {
    const item = localStorage.getItem('user');
    return item ? (JSON.parse(item) as UserEntity) : null;
  }

  setUser(user: UserEntity) {
    const userJson = JSON.stringify(user);
    localStorage.setItem('user', userJson);
  }

  removeUser() {
    localStorage.removeItem('user');
  }

  clear() {
    this.removeToken();
    this.removeUser();
  }
}
