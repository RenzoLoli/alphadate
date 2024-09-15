import { Component, inject, signal } from '@angular/core';
import { toSignal } from '@angular/core/rxjs-interop';
import {
  FormControl,
  FormGroup,
  ReactiveFormsModule,
  Validators,
} from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import { MatDialogModule, MatDialogRef } from '@angular/material/dialog';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { rxMethod } from '@ngrx/signals/rxjs-interop';
import { catchError, map, pipe, switchMap, tap } from 'rxjs';
import { DateIdeaCreateRequest } from '../../models/date-idea-create.request';
import { DateIdeaEntity } from '../../models/date-idea.entity';
import { DateIdeaService } from '../../services/date-idea.service';

const COMPONENTS: Array<any> = [];
const MATERIAL: Array<any> = [
  MatDialogModule,
  MatFormFieldModule,
  MatInputModule,
  MatButtonModule,
];

@Component({
  selector: 'app-idea-add-form-dialog',
  standalone: true,
  imports: [ReactiveFormsModule, COMPONENTS, MATERIAL],
  templateUrl: './idea-add-form-dialog.component.html',
  styleUrl: './idea-add-form-dialog.component.css',
})
export class IdeaAddFormDialogComponent {
  readonly dialogRef = inject(MatDialogRef<IdeaAddFormDialogComponent>);

  dateIdeaService = inject(DateIdeaService);

  submitDateIdeaCreation = rxMethod<DateIdeaCreateRequest>(
    pipe(
      switchMap((value) => {
        return this.dateIdeaService.create(value);
      }),
      tap((dateIdea: DateIdeaEntity) => {
        this.dialogRef.close(dateIdea);
      }),
      catchError((error, caught) => {
        this.reqError.set(error.message);
        return caught;
      }),
    ),
  );

  reqError = signal('');

  addIdeaForm = new FormGroup({
    idea: new FormControl('', {
      nonNullable: true,
      validators: [
        Validators.required,
        Validators.minLength(3),
        Validators.maxLength(20),
      ],
    }),
    description: new FormControl('', {
      nonNullable: true,
      validators: [
        Validators.required,
        Validators.minLength(3),
        Validators.maxLength(200),
      ],
    }),
  });

  get idea() {
    return this.addIdeaForm.get('idea');
  }

  get description() {
    return this.addIdeaForm.get('description');
  }

  disabled = toSignal(
    this.addIdeaForm.statusChanges.pipe(
      map((status) => {
        return status != 'VALID';
      }),
    ),
    {
      initialValue: true,
    },
  );

  onCancel() {
    this.dialogRef.close();
  }

  onSubmit() {
    if (this.addIdeaForm.invalid) return;

    this.submitDateIdeaCreation(
      this.addIdeaForm.value as DateIdeaCreateRequest,
    );
  }
}
