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
import { AlphabetCreateRequest } from '../../models/alphabet-create.request';
import { AlphabetService } from '../../services/alphabet.service';
import { AuthStore } from '../../../user/store/auth.store';

const COMPONENTS: Array<any> = [];
const MATERIAL: Array<any> = [
  MatDialogModule,
  MatFormFieldModule,
  MatInputModule,
  MatButtonModule,
];
const ADITIONAL: Array<any> = [ReactiveFormsModule];

@Component({
  selector: 'app-alphabet-add-form-dialog',
  standalone: true,
  imports: [COMPONENTS, MATERIAL, ADITIONAL],
  templateUrl: './alphabet-add-form-dialog.component.html',
  styleUrl: './alphabet-add-form-dialog.component.css',
})
export class AlphabetAddFormDialogComponent {
  readonly dialogRef = inject(MatDialogRef<AlphabetAddFormDialogComponent>);

  alphabetService = inject(AlphabetService);
  authStore = inject(AuthStore);

  reqError = signal('');

  addAlphabetBaseForm = new FormGroup({
    title: new FormControl('', {
      nonNullable: true,
      validators: [
        Validators.required,
        Validators.minLength(3),
        Validators.maxLength(50),
      ],
    }),
  });

  get title() {
    return this.addAlphabetBaseForm.get('title');
  }

  submitAlphabetBaseCreation = rxMethod<AlphabetCreateRequest>(
    pipe(
      switchMap((value) => {
        return this.alphabetService.create(value);
      }),
      tap((alphabetBase) => {
        this.dialogRef.close(alphabetBase.id);
      }),
      catchError((error, caught) => {
        this.reqError.set(error.message);
        return caught;
      }),
    ),
  );

  disabled = toSignal(
    this.addAlphabetBaseForm.statusChanges.pipe(
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
    if (this.addAlphabetBaseForm.invalid) return;
    const userId = this.authStore.getUser()?.id;
    if (!userId) {
      this.reqError.set('You must be logged in to create an alphabet');
      return;
    }

    const createAlphabetRequest: AlphabetCreateRequest = {
      title: this.addAlphabetBaseForm.value.title!,
      user_id: userId!,
    };

    this.submitAlphabetBaseCreation(createAlphabetRequest);
  }
}
