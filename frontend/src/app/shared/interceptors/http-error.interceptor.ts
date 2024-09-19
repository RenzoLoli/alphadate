import { HttpInterceptorFn } from '@angular/common/http';
import { catchError, throwError } from 'rxjs';

type ErrorEvent = {
  status: number;
  message: string;
};

export const httpErrorInterceptor: HttpInterceptorFn = (req, next) => {
  return next(req).pipe(
    catchError((error) => {
      let errorMessage = 'Something went wrong. Please try again later.';

      if (error.error instanceof ErrorEvent) {
        // client-side error
        console.log(`An error occurred: ${error.message}`);
      } else {
        // server-side error
        errorMessage = error.error.message;

        console.log(
          `Backend returned core ${error.status}, body was ${JSON.stringify(error.error)}`,
        );
      }

      return throwError(() => {
        return {
          status: error.status,
          message: errorMessage,
        } as ErrorEvent;
      });
    }),
  );
};
