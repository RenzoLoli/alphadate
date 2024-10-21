import { Component, inject } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import {
  MAT_DIALOG_DATA,
  MatDialogModule,
  MatDialogRef,
} from '@angular/material/dialog';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { AlphabetStore } from '../../../alphabet/store/alphabet.store';
import { DateIdeaEntity } from '../../models/date-idea.entity';

const COMPONENTS: Array<any> = [];
const MATERIAL: Array<any> = [
  MatDialogModule,
  MatFormFieldModule,
  MatInputModule,
  MatButtonModule,
];

type DialogData = DateIdeaEntity;

@Component({
  selector: 'app-confirm-add-idea-alphabet-dialog',
  standalone: true,
  imports: [COMPONENTS, MATERIAL],
  templateUrl: './confirm-add-idea-alphabet-dialog.component.html',
  styleUrl: './confirm-add-idea-alphabet-dialog.component.css',
})
export class ConfirmAddIdeaAlphabetDialogComponent {
  readonly dialogRef = inject(
    MatDialogRef<ConfirmAddIdeaAlphabetDialogComponent>,
  );

  alphabetStore = inject(AlphabetStore);

  get alphabetTitle() {
    return this.alphabetStore.getCurrentAlphabet()?.title;
  }

  get dateIdea(): DateIdeaEntity {
    return this.data;
  }

  data = inject<DialogData>(MAT_DIALOG_DATA);

  onCancel() {
    this.dialogRef.close(false);
  }

  onSubmit() {
    return true;
  }
}
