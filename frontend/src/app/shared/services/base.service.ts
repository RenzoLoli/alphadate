import { environment } from '../../../environments/environment';
import { HttpClient } from '@angular/common/http';

export class BaseService {
  basePath: string = `${environment.apiUrl}`;
  resourceEndPoint: string = '/resources';

  constructor(protected http: HttpClient) {}

  protected resourcePath() {
    return `${environment.apiUrl}${this.resourceEndPoint}`;
  }
}
