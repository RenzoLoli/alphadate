import { Component, computed, inject, OnInit, signal } from '@angular/core';
import {
  FormControl,
  FormGroup,
  ReactiveFormsModule,
  Validators,
} from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import {
  MAT_DIALOG_DATA,
  MatDialogModule,
  MatDialogRef,
} from '@angular/material/dialog';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { rxMethod } from '@ngrx/signals/rxjs-interop';
import { catchError, of, pipe, switchMap, tap, throwError } from 'rxjs';
import { DateIdeaCreateRequest } from '../../models/date-idea-create.request';
import {
  checkUnnecessaryDateIdeaUpdateRequest,
  DateIdeaUpdateRequest,
} from '../../models/date-idea-update.request';
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
  selector: 'app-idea-update-form-dialog',
  standalone: true,
  imports: [ReactiveFormsModule, COMPONENTS, MATERIAL],
  templateUrl: './idea-update-form-dialog.component.html',
  styleUrl: './idea-update-form-dialog.component.css',
})
export class IdeaUpdateFormDialogComponent implements OnInit {
  readonly dialogRef = inject(MatDialogRef<IdeaUpdateFormDialogComponent>);

  dateIdeaService = inject(DateIdeaService);

  dateIdeaInput = inject<DateIdeaEntity>(MAT_DIALOG_DATA);

  submitDateIdeaUpdate = rxMethod<DateIdeaUpdateRequest>(
    pipe(
      switchMap((value) => {
        if (!checkUnnecessaryDateIdeaUpdateRequest(value)) {
          return throwError(() => new Error('Invalid request'));
        }
        return this.dateIdeaService.update(this.dateIdeaInput.id, value);
      }),
      tap((dateIdea: DateIdeaEntity) => {
        this.dialogRef.close(dateIdea);
      }),
      catchError((error) => {
        this.reqError.set(error.message);
        return of(null);
      }),
    ),
  );

  invalidForm = signal(false);
  sameInputForm = signal(true);
  reqError = signal('');

  updateIdeaForm = new FormGroup({
    idea: new FormControl(this.dateIdeaInput.idea, {
      nonNullable: true,
      validators: [
        Validators.required,
        Validators.minLength(3),
        Validators.maxLength(20),
      ],
    }),
    description: new FormControl(this.dateIdeaInput.description, {
      nonNullable: true,
      validators: [
        Validators.required,
        Validators.minLength(3),
        Validators.maxLength(200),
      ],
    }),
  });

  get idea() {
    return this.updateIdeaForm.get('idea');
  }

  get description() {
    return this.updateIdeaForm.get('description');
  }

  disabled = computed(() => {
    return this.invalidForm() || this.sameInputForm();
  });

  ngOnInit(): void {
    this.updateIdeaForm.statusChanges.subscribe((status) => {
      this.invalidForm.set(status != 'VALID');
    });

    this.updateIdeaForm.valueChanges.subscribe((value) => {
      this.sameInputForm.set(
        value.idea === this.dateIdeaInput.idea &&
          value.description === this.dateIdeaInput.description,
      );
    });
  }

  onCancel() {
    this.dialogRef.close();
  }

  onSubmit() {
    if (this.updateIdeaForm.invalid) return;

    this.submitDateIdeaUpdate(
      this.updateIdeaForm.value as DateIdeaCreateRequest,
    );
  }
}
