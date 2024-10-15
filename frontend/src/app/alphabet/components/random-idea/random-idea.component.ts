import { Component, inject, OnInit, signal } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatDialogModule, MatDialogRef } from '@angular/material/dialog';
import { rxMethod } from '@ngrx/signals/rxjs-interop';
import { catchError, pipe, switchMap, tap, throwError } from 'rxjs';
import { DateIdeaEntity } from '../../../date-idea/models/date-idea.entity';
import { DateIdeaService } from '../../../date-idea/services/date-idea.service';
import { AlphabetStore } from '../../store/alphabet.store';

const COMPONENTS: Array<any> = [];
const MATERIAL: Array<any> = [MatDialogModule, MatButtonModule];

@Component({
  selector: 'app-random-idea',
  standalone: true,
  imports: [COMPONENTS, MATERIAL],
  templateUrl: './random-idea.component.html',
  styleUrl: './random-idea.component.css',
})
export class RandomIdeaComponent implements OnInit {
  private dialogRef = inject(MatDialogRef<RandomIdeaComponent>);

  dateIdea = signal<DateIdeaEntity | null>(null);
  error = signal<string | null>(null);
  dateIdeaService = inject(DateIdeaService);
  alphabetStore = inject(AlphabetStore);

  $randomIdea = rxMethod<void>(
    pipe(
      switchMap(() => {
        const alphabetId = this.alphabetStore.getCurrentAlphabetId();
        if (!alphabetId) return throwError(() => new Error('Invalid alphabet'));

        return this.dateIdeaService.randomIdea(alphabetId);
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
