import { Component, inject } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import {
  MAT_DIALOG_DATA,
  MatDialogModule,
  MatDialogRef,
} from '@angular/material/dialog';

const COMPONENTS: Array<any> = [];
const MATERIAL: Array<any> = [MatDialogModule, MatButtonModule];

@Component({
  selector: 'app-confirmation-dialog',
  standalone: true,
  imports: [COMPONENTS, MATERIAL],
  templateUrl: './confirmation-dialog.component.html',
  styleUrl: './confirmation-dialog.component.css',
})
export class ConfirmationDialogComponent {
  readonly dialogRef = inject(MatDialogRef<ConfirmationDialogComponent>);
  data = inject<{ message: string }>(MAT_DIALOG_DATA);

  onExit() {
    this.dialogRef.close(false);
  }

  onYes() {
    this.dialogRef.close(true);
  }
}
