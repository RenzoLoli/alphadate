import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable, retry, tap } from 'rxjs';
import { BaseService } from '../../shared/services/base.service';
import { DateIdeaCreateRequest } from '../models/date-idea-create.request';
import { DateIdeaUpdateRequest } from '../models/date-idea-update.request';
import { DateIdeaEntity } from '../models/date-idea.entity';
import { DateIdeaRandomRequest } from '../models/date-idea-random.request';

@Injectable({
  providedIn: 'root',
})
export class DateIdeaService extends BaseService {
  constructor(http: HttpClient) {
    super(http);
    this.resourceEndPoint = '/date-idea';
  }

  getAll(): Observable<DateIdeaEntity[]> {
    const path = `${this.resourcePath()}/all`;
    return this.http.get<DateIdeaEntity[]>(path).pipe(retry(2));
  }

  getById(id: string): Observable<DateIdeaEntity> {
    const path = `${this.resourcePath()}/${id}`;
    return this.http.get<DateIdeaEntity>(path).pipe(retry(2));
  }

  addTag(ideaId: string, tagId: string): Observable<DateIdeaEntity> {
    const path = `${this.resourcePath()}/${ideaId}/tag`;
    return this.http
      .post<DateIdeaEntity>(path, {
        tag_id: tagId,
      })
      .pipe(retry(2));
  }

  removeTag(ideaId: string, tagId: string): Observable<DateIdeaEntity> {
    const path = `${this.resourcePath()}/${ideaId}/tag`;
    return this.http
      .delete<DateIdeaEntity>(path, {
        body: {
          tag_id: tagId,
        },
      })
      .pipe(retry(2));
  }

  update(
    id: string,
    dateIdeaUpdateRequest: DateIdeaUpdateRequest,
  ): Observable<DateIdeaEntity> {
    const path = `${this.resourcePath()}/${id}`;
    return this.http
      .put<DateIdeaEntity>(path, dateIdeaUpdateRequest)
      .pipe(retry(2));
  }

  create(
    dateIdeaCreateRequest: DateIdeaCreateRequest,
  ): Observable<DateIdeaEntity> {
    const path = `${this.resourcePath()}`;
    return this.http
      .post<DateIdeaEntity>(path, dateIdeaCreateRequest)
      .pipe(retry(2));
  }

  delete(id: string): Observable<void> {
    const path = `${this.resourcePath()}/${id}`;
    return this.http.delete<void>(path).pipe(retry(2));
  }

  randomIdea(request: DateIdeaRandomRequest): Observable<DateIdeaEntity> {
    const { alphabetId, excludeActive } = request;
    const path = `${this.resourcePath()}/random/${alphabetId}`;
    return this.http
      .get<DateIdeaEntity>(path, {
        params: {
          exclude_active: excludeActive,
        },
      })
      .pipe(retry(2));
  }
}
