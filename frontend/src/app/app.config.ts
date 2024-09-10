import { ApplicationConfig, provideZoneChangeDetection } from '@angular/core';
import { provideRouter } from '@angular/router';

import { provideHttpClient, withInterceptors } from '@angular/common/http';
import { provideAnimationsAsync } from '@angular/platform-browser/animations/async';
import { routes } from './app.routes';
import { httpBearerInterceptor } from './shared/interceptors/http-bearer.interceptor';
import { httpDefaultOptionsInterceptor } from './shared/interceptors/http-default-options.interceptor';
import { httpErrorInterceptor } from './shared/interceptors/http-error.interceptor';

export const appConfig: ApplicationConfig = {
  providers: [
    provideZoneChangeDetection({ eventCoalescing: true }),
    provideRouter(routes),
    provideHttpClient(
      withInterceptors([
        httpDefaultOptionsInterceptor,
        httpBearerInterceptor,
        httpErrorInterceptor,
      ]),
    ),
    provideAnimationsAsync(),
  ],
};
