import { Injectable } from '@angular/core';
import { BaseService } from '../../shared/services/base.service';
import { HttpClient } from '@angular/common/http';
import { Observable, retry } from 'rxjs';
import { TagUpdateRequest } from '../models/tag-update.request';
import { TagCreateRequest } from '../models/tag-create.request';
import { TagEntity } from '../models/tag.entity';

@Injectable({
  providedIn: 'root',
})
export class TagService extends BaseService {
  constructor(http: HttpClient) {
    super(http);
    this.resourceEndPoint = '/tag';
  }

  getAll(): Observable<TagEntity[]> {
    const path = `${this.resourcePath()}/all`;
    return this.http.get<TagEntity[]>(path).pipe(retry(2));
  }

  getById(id: string): Observable<TagEntity> {
    const path = `${this.resourcePath()}/${id}`;
    return this.http.get<TagEntity>(path).pipe(retry(2));
  }

  update(tagUpdateRequest: TagUpdateRequest): Observable<void> {
    const path = `${this.resourcePath()}`;
    return this.http.put<void>(path, tagUpdateRequest).pipe(retry(2));
  }

  create(tagCreateRequest: TagCreateRequest): Observable<TagEntity> {
    const path = `${this.resourcePath()}`;
    return this.http.post<TagEntity>(path, tagCreateRequest).pipe(retry(2));
  }

  delete(id: string): Observable<void> {
    const path = `${this.resourcePath()}/${id}`;
    return this.http.delete<void>(path).pipe(retry(2));
  }
}
