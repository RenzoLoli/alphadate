import { HttpInterceptorFn } from '@angular/common/http';
import { catchError, throwError } from 'rxjs';

export const httpErrorInterceptor: HttpInterceptorFn = (req, next) => {
  return next(req).pipe(
    catchError((error) => {
      let errorMessage = '';
      if (error.error instanceof ErrorEvent) {
        // client-side error
        errorMessage = 'Something went wrong. Please try again later.';
        console.error(`An error occurred: ${error.error.message}`);
      } else {
        // server-side error
        errorMessage = error.error.message;
        console.log(
          `Backend returned core ${error.status}, body was ${JSON.stringify(error.error)}`,
        );
      }
      return throwError(() => {
        return new Error(errorMessage);
      });
    }),
  );
};
