import { Component, inject, model, OnInit, signal } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import { MatCheckboxModule } from '@angular/material/checkbox';
import { MatDialogModule, MatDialogRef } from '@angular/material/dialog';
import { rxMethod } from '@ngrx/signals/rxjs-interop';
import { catchError, pipe, switchMap, tap, throwError } from 'rxjs';
import { DateIdeaRandomRequest } from '../../../date-idea/models/date-idea-random.request';
import { DateIdeaEntity } from '../../../date-idea/models/date-idea.entity';
import { DateIdeaService } from '../../../date-idea/services/date-idea.service';
import { AlphabetStore } from '../../store/alphabet.store';

const COMPONENTS: Array<any> = [];
const MATERIAL: Array<any> = [
  MatDialogModule,
  MatButtonModule,
  MatCheckboxModule,
];
const ADITIONAL: Array<any> = [FormsModule];

@Component({
  selector: 'app-random-idea',
  standalone: true,
  imports: [COMPONENTS, MATERIAL, ADITIONAL],
  templateUrl: './random-idea.component.html',
  styleUrl: './random-idea.component.css',
})
export class RandomIdeaComponent implements OnInit {
  private dialogRef = inject(MatDialogRef<RandomIdeaComponent>);

  dateIdea = signal<DateIdeaEntity | null>(null);
  error = signal<string | null>(null);
  dateIdeaService = inject(DateIdeaService);
  alphabetStore = inject(AlphabetStore);

  excludeMode = model(false);

  $randomIdea = rxMethod<void>(
    pipe(
      switchMap(() => {
        const alphabetId = this.alphabetStore.getCurrentAlphabetId();
        if (!alphabetId) return throwError(() => new Error('Invalid alphabet'));

        const excludeActive = this.excludeMode();

        const request: DateIdeaRandomRequest = {
          alphabetId,
          excludeActive,
        };
        return this.dateIdeaService.randomIdea(request);
      }),
      tap((dateIdea) => {
        this.dateIdea.set(dateIdea);
      }),
      catchError((_error, caught) => {
        const error = typeof _error === 'string' ? _error : _error.message;
        this.error.set(error);
        return caught;
      }),
    ),
  );

  ngOnInit(): void {
    this.$randomIdea();
  }

  onCancel() {
    this.dialogRef.close(null);
  }

  onSubmit() {
    this.dialogRef.close({
      dateIdea: this.dateIdea(),
    });
  }

  onSpin() {
    this.$randomIdea();
  }
}
