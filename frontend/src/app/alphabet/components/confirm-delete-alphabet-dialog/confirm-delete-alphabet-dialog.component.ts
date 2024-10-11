import { Component, inject } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import {
  MAT_DIALOG_DATA,
  MatDialogModule,
  MatDialogRef,
} from '@angular/material/dialog';
import { AlphabetEntity } from '../../models/alphabet.entity';

const COMPONENTS: Array<any> = [];
const MATERIAL: Array<any> = [MatDialogModule, MatButtonModule];

interface DialogData {
  alphabet: AlphabetEntity;
}

@Component({
  selector: 'app-confirm-delete-alphabet-dialog',
  standalone: true,
  imports: [COMPONENTS, MATERIAL],
  templateUrl: './confirm-delete-alphabet-dialog.component.html',
  styleUrl: './confirm-delete-alphabet-dialog.component.css',
})
export class ConfirmDeleteAlphabetDialogComponent {
  private dialogRef = inject(
    MatDialogRef<ConfirmDeleteAlphabetDialogComponent>,
  );

  data = inject<DialogData>(MAT_DIALOG_DATA);

  get title() {
    return this.data.alphabet.title;
  }

  onCancel() {
    this.dialogRef.close(false);
  }

  onSubmit() {
    this.dialogRef.close(true);
  }
}
