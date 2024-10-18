import { Component, inject, model, OnInit, signal } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import { MatCheckboxModule } from '@angular/material/checkbox';
import {
  MAT_DIALOG_DATA,
  MatDialogModule,
  MatDialogRef,
} from '@angular/material/dialog';
import { rxMethod } from '@ngrx/signals/rxjs-interop';
import { catchError, pipe, switchMap, tap, throwError } from 'rxjs';
import { AlphabetStore } from '../../../alphabet/store/alphabet.store';
import { DateIdeaRandomRequest } from '../../../date-idea/models/date-idea-random.request';
import { DateIdeaEntity } from '../../../date-idea/models/date-idea.entity';
import { DateIdeaService } from '../../../date-idea/services/date-idea.service';

const COMPONENTS: Array<any> = [];
const MATERIAL: Array<any> = [
  MatDialogModule,
  MatButtonModule,
  MatCheckboxModule,
];
const ADITIONAL: Array<any> = [FormsModule];

type Data = Array<DateIdeaEntity>;

@Component({
  selector: 'app-random-date-idea-dialog',
  standalone: true,
  imports: [COMPONENTS, MATERIAL, ADITIONAL],
  templateUrl: './random-date-idea-dialog.component.html',
  styleUrl: './random-date-idea-dialog.component.css',
})
export class RandomDateIdeaDialogComponent implements OnInit {
  private dialogRef = inject(MatDialogRef<RandomDateIdeaDialogComponent>);

  dateIdea = signal<DateIdeaEntity | null>(null);
  error = signal<string | null>(null);
  dateIdeaService = inject(DateIdeaService);
  alphabetStore = inject(AlphabetStore);

  data: Data = inject(MAT_DIALOG_DATA);

  spinIdeas = () => {
    const dateIdeas = this.data;
    const rIndex = Math.floor(Math.random() * dateIdeas.length);
    const idea = dateIdeas[rIndex];
    this.dateIdea.set(idea);
  };

  ngOnInit(): void {
    this.spinIdeas();
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
    this.spinIdeas();
  }
}
