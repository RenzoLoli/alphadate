import { HttpClient } from '@angular/common/http';
import { Observable, retry } from 'rxjs';
import { BaseService } from '../../shared/services/base.service';
import { AlphabetAddDateIdeaRequest } from '../models/alphabet-add-date-idea.request';
import { AlphabetCreateRequest } from '../models/alphabet-create.request';
import { AlphabetRemoveDateIdeaRequest } from '../models/alphabet-remove-date-idea.request';
import { AlphabetEntity } from '../models/alphabet.entity';
import { Injectable } from '@angular/core';
import { AlphabetBaseEntity } from '../models/alphabet-base.entity';
import { AlphabetToggleCompleteDateRequest } from '../models/alphabet-toggle-complete-date-request';
import { UserDateIsCompletedResponse } from '../models/user_date_is_completed.response';

@Injectable({
  providedIn: 'root',
})
export class AlphabetService extends BaseService {
  constructor(http: HttpClient) {
    super(http);

    this.resourceEndPoint = '/alphabet';
  }

  getAll(): Observable<AlphabetEntity[]> {
    const path = `${this.resourcePath()}/all`;
    return this.http.get<AlphabetEntity[]>(path).pipe(retry(2));
  }

  getAllBase(userId: string): Observable<AlphabetBaseEntity[]> {
    const path = `${this.resourcePath()}/all/${userId}/base`;
    return this.http.get<AlphabetBaseEntity[]>(path).pipe(retry(2));
  }

  getById(id: string): Observable<AlphabetEntity> {
    const path = `${this.resourcePath()}/${id}`;
    return this.http.get<AlphabetEntity>(path).pipe(retry(2));
  }

  create(
    alphabetCreateRequest: AlphabetCreateRequest,
  ): Observable<AlphabetBaseEntity> {
    const path = `${this.resourcePath()}`;
    return this.http
      .post<AlphabetBaseEntity>(path, alphabetCreateRequest)
      .pipe(retry(2));
  }

  delete(id: string): Observable<AlphabetBaseEntity> {
    const path = `${this.resourcePath()}/${id}`;
    return this.http.delete<AlphabetBaseEntity>(path).pipe(retry(2));
  }

  addDateIdeaTo(body: AlphabetAddDateIdeaRequest): Observable<void> {
    const path = `${this.resourcePath()}/${body.id}/date-idea/${body.date_idea_id}`;
    return this.http.post<void>(path, {}).pipe(retry(2));
  }

  removeDateIdeaTo(body: AlphabetRemoveDateIdeaRequest): Observable<void> {
    const path = `${this.resourcePath()}/${body.id}/date-idea/${body.date_idea_id}`;
    return this.http.delete<void>(path).pipe(retry(2));
  }

  toggleCompleteDate(
    body: AlphabetToggleCompleteDateRequest,
  ): Observable<UserDateIsCompletedResponse> {
    const path = `${this.resourcePath()}/${body.id}/complete/${body.user_date}`;
    return this.http
      .patch<UserDateIsCompletedResponse>(path, {})
      .pipe(retry(2));
  }
}
