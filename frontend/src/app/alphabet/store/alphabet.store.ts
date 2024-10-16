import { computed, inject } from '@angular/core';
import {
  patchState,
  signalStore,
  withComputed,
  withHooks,
  withMethods,
  withState,
} from '@ngrx/signals';
import { rxMethod } from '@ngrx/signals/rxjs-interop';
import { catchError, of, pipe, switchMap, tap, throwError } from 'rxjs';
import { AlphabetAddDateIdeaRequest } from '../models/alphabet-add-date-idea.request';
import { AlphabetRemoveDateIdeaRequest } from '../models/alphabet-remove-date-idea.request';
import { AlphabetToggleCompleteDateRequest } from '../models/alphabet-toggle-complete-date-request';
import { AlphabetEntity } from '../models/alphabet.entity';
import { UserDateEntity } from '../models/user-date.entity';
import { AlphabetService } from '../services/alphabet.service';

export interface AlphabetState {
  currentAlphabetId: string | null;
  alphabet: AlphabetEntity | null;
  error: string | null;
}

const initialState: AlphabetState = {
  currentAlphabetId: null,
  alphabet: null,
  error: null,
};

export const AlphabetStore = signalStore(
  { providedIn: 'root' },
  withState(initialState),
  withComputed((state) => ({
    getCurrentAlphabetId: computed(() => state.currentAlphabetId()),
    getCurrentAlphabet: computed(() => state.alphabet()),
    getCurrentActiveLetters: computed(() => {
      const alphabet = state.alphabet();
      const mapped = new Map<string, boolean>();
      'abcdefghijklmnopqrstuvwxyz'.split('').forEach((letter) => {
        mapped.set(letter, false);
      });

      if (!alphabet) return mapped;

      alphabet.user_dates.forEach((date) => {
        mapped.set(date.letter, true);
      });

      return mapped;
    }),
    getCurrentUserDatesMap: computed(() => {
      const alphabet = state.alphabet();
      const mapped = new Map<string, UserDateEntity | null>();
      'abcdefghijklmnopqrstuvwxyz'.split('').forEach((letter) => {
        mapped.set(letter, null);
      });

      if (!alphabet) return mapped;

      alphabet.user_dates.forEach((date) => {
        mapped.set(date.letter, date);
      });

      return mapped;
    }),
    getError: computed(() => state.error()),
  })),
  withMethods((store, alphabetService = inject(AlphabetService)) => ({
    clear: () => {
      localStorage.removeItem('currentAlphabetId');
      patchState(store, {
        alphabet: null,
        currentAlphabetId: null,
        error: null,
      });
    },
    initAlphabet: rxMethod<void>(
      pipe(
        switchMap(() => {
          const alphabetId = localStorage.getItem('currentAlphabetId');
          if (!alphabetId) {
            return throwError(() => new Error('Invalid alphabet id'));
          }

          return alphabetService.getById(alphabetId);
        }),
        tap((alphabet) => {
          patchState(store, {
            alphabet,
            currentAlphabetId: alphabet.id,
            error: null,
          });
        }),
        catchError((error, caught) => {
          console.log(error);
          patchState(store, { error: error.message });
          return caught;
        }),
      ),
    ),
    // chooseAlphabet: rxMethod<string>(
    //   pipe(
    //     switchMap((alphabetId) => alphabetService.getById(alphabetId)),
    //     tap((entity) => {
    //       localStorage.setItem('currentAlphabetId', entity.id);
    //       patchState(store, {
    //         alphabet: entity,
    //         currentAlphabetId: entity.id,
    //         error: null,
    //       });
    //     }),
    //     catchError((error, caught) => {
    //       patchState(store, { error: error.message });
    //       return caught;
    //     }),
    //   ),
    // ),
    chooseAlphabet: (alphabetId: string) => {
      return of(alphabetId).pipe(
        switchMap(() => alphabetService.getById(alphabetId)),
        tap((entity) => {
          localStorage.setItem('currentAlphabetId', entity.id);
          patchState(store, {
            alphabet: entity,
            currentAlphabetId: entity.id,
            error: null,
          });
        }),
      );
    },
    deleteAlphabet: rxMethod<string>(
      pipe(
        switchMap((alphabetId) => alphabetService.delete(alphabetId)),
        tap((_entity) => {
          patchState(store, {
            alphabet: null,
            currentAlphabetId: null,
            error: null,
          });
          localStorage.removeItem('currentAlphabetId');
          window.location.reload();
        }),
        catchError((error, caught) => {
          patchState(store, { error: error.message });
          return caught;
        }),
      ),
    ),
    // addDateIdea: rxMethod<string>(
    //   pipe(
    //     switchMap((dateIdeaId) => {
    //       const alphabetId = store.getCurrentAlphabetId();
    //
    //       if (!alphabetId)
    //         return throwError(() => new Error('Invalid alphabet id'));
    //
    //       const request: AlphabetAddDateIdeaRequest = {
    //         id: alphabetId,
    //         date_idea_id: dateIdeaId,
    //       };
    //       return alphabetService.addDateIdeaTo(request);
    //     }),
    //     tap(() => {
    //       patchState(store, { error: null });
    //     }),
    //     catchError((error, caught) => {
    //       patchState(store, { error: error.message });
    //       return caught;
    //     }),
    //   ),
    // ),
    addDateIdea(dateIdeaId: string) {
      return of(null).pipe(
        switchMap(() => {
          const alphabetId = store.getCurrentAlphabetId();
          if (!alphabetId)
            return throwError(() => new Error('Invalid alphabet id'));
          const request: AlphabetAddDateIdeaRequest = {
            id: alphabetId,
            date_idea_id: dateIdeaId,
          };
          return alphabetService.addDateIdeaTo(request);
        }),
        tap(() => {
          patchState(store, { error: null });
        }),
      );
    },
    removeDateIdea(dateIdeaId: string) {
      return of(null).pipe(
        switchMap(() => {
          const alphabetId = store.getCurrentAlphabetId();
          if (!alphabetId)
            return throwError(() => new Error('Invalid alphabet id'));
          const request: AlphabetRemoveDateIdeaRequest = {
            id: alphabetId,
            date_idea_id: dateIdeaId,
          };
          return alphabetService.removeDateIdeaTo(request);
        }),
        tap(() => {
          patchState(store, { error: null });
        }),
      );
    },
    // removeDateIdea: rxMethod<string>(
    //   pipe(
    //     switchMap((dateIdeaId) => {
    //       const alphabetId = store.getCurrentAlphabetId();
    //
    //       if (!alphabetId)
    //         return throwError(() => new Error('Invalid alphabet id'));
    //
    //       const request: AlphabetRemoveDateIdeaRequest = {
    //         id: alphabetId,
    //         date_idea_id: dateIdeaId,
    //       };
    //       return alphabetService.removeDateIdeaTo(request);
    //     }),
    //     tap(() => {
    //       patchState(store, { error: null });
    //       window.location.href = '/';
    //     }),
    //     catchError((error, caught) => {
    //       patchState(store, { error: error.message });
    //       return caught;
    //     }),
    //   ),
    // ),
    toggleCompleteDate(userDateId: string) {
      return of(userDateId).pipe(
        switchMap(() => {
          const alphabetId = store.getCurrentAlphabetId();
          if (!alphabetId)
            return throwError(() => new Error('Invalid alphabet id'));

          const request: AlphabetToggleCompleteDateRequest = {
            id: alphabetId,
            user_date: userDateId,
          };

          return alphabetService.toggleCompleteDate(request);
        }),
        tap(() => {
          patchState(store, { error: null });
        }),
      );
    },
  })),
  withHooks((state) => ({
    onInit: () => {
      state.initAlphabet();
    },
    onDestroy: () => {},
  })),
);
