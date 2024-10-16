import {
  AfterViewInit,
  Component,
  effect,
  inject,
  OnInit,
  signal,
  viewChild,
} from '@angular/core';
import { ReactiveFormsModule } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import { MatCheckbox, MatCheckboxModule } from '@angular/material/checkbox';
import { MatChipsModule } from '@angular/material/chips';
import { MatDialog } from '@angular/material/dialog';
import { MatDividerModule } from '@angular/material/divider';
import { MatExpansionModule } from '@angular/material/expansion';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatIcon, MatIconModule } from '@angular/material/icon';
import {
  MatSelect,
  MatSelectChange,
  MatSelectModule,
} from '@angular/material/select';
import { ActivatedRoute, Router } from '@angular/router';
import { rxMethod } from '@ngrx/signals/rxjs-interop';
import { catchError, finalize, pipe, switchMap, tap, throwError } from 'rxjs';
import { DateIdeaEntity } from '../../../date-idea/models/date-idea.entity';
import { AuthStore } from '../../../user/store/auth.store';
import { AlphabetAddFormDialogComponent } from '../../components/alphabet-add-form-dialog/alphabet-add-form-dialog.component';
import { ConfirmDeleteAlphabetDialogComponent } from '../../components/confirm-delete-alphabet-dialog/confirm-delete-alphabet-dialog.component';
import { AlphabetBaseEntity } from '../../models/alphabet-base.entity';
import { UserDateEntity } from '../../models/user-date.entity';
import { AlphabetService } from '../../services/alphabet.service';
import { AlphabetStore } from '../../store/alphabet.store';
import { DateIdeaService } from '../../../date-idea/services/date-idea.service';
import { RandomIdeaComponent } from '../../components/random-idea/random-idea.component';

const COMPONENTS: any[] = [];
const MATERIAL: any[] = [
  MatExpansionModule,
  MatSelectModule,
  MatFormFieldModule,
  MatIconModule,
  MatButtonModule,
  MatChipsModule,
  MatDividerModule,
  MatCheckboxModule,
];
const ADITIONAL = [ReactiveFormsModule];

@Component({
  selector: 'app-home',
  standalone: true,
  imports: [MATERIAL, COMPONENTS, ADITIONAL],
  templateUrl: './home.component.html',
  styleUrl: './home.component.css',
})
export class HomeComponent implements OnInit, AfterViewInit {
  authStore = inject(AuthStore);
  alphabetStore = inject(AlphabetStore);

  router = inject(Router);

  addAlphabetBaseDialog = inject(MatDialog);
  confirmDeleteAlphabetDialog = inject(MatDialog);
  randomIdeaDialog = inject(MatDialog);

  alphabetTitles = signal<AlphabetBaseEntity[]>([]);

  get alphabetDates() {
    return this.alphabetStore.getCurrentUserDatesMap();
  }

  get currAlphabetId() {
    return this.alphabetStore.currentAlphabetId();
  }

  alphabetService = inject(AlphabetService);
  dateIdeaService = inject(DateIdeaService);

  $getAllTitles = rxMethod<void>(
    pipe(
      switchMap(() => {
        const userId = this.authStore.getUser()?.id;
        if (!userId) return throwError(() => new Error('Invalid user id'));

        return this.alphabetService.getAllBase(userId);
      }),
      tap((alphabets) => {
        this.alphabetTitles.set(alphabets);
      }),
      catchError((_error, caught) => {
        return caught;
      }),
    ),
  );

  selectHtml = viewChild<MatSelect>('selectHtml');

  _ = effect(() => {
    const id = this.alphabetStore.getCurrentAlphabetId();
    const selectHtml = this.selectHtml();

    if (id && selectHtml && !selectHtml.value) {
      selectHtml.value = id;
    }
  });

  constructor(private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.$getAllTitles();
  }

  ngAfterViewInit(): void {
    this.route.queryParams.subscribe((params) => {
      const letter = params['letter'];
      if (!letter) return;

      const row = document.getElementById('panel-' + letter);
      if (!row) return;
      row.classList.add('added-row');
      row.scrollIntoView({
        behavior: 'auto',
        block: 'start',
      });
    });
  }

  get dates() {
    return this.alphabetStore.getCurrentAlphabet()?.user_dates || [];
  }

  get currAlphabetTitle() {
    return this.alphabetStore.getCurrentAlphabet()?.title;
  }

  onSelectTitle(event: MatSelectChange) {
    this.alphabetStore.chooseAlphabet(event.value).subscribe(() => {
      this.$getAllTitles();
    });
  }

  onAddAlphabet() {
    const dialogRef = this.addAlphabetBaseDialog.open(
      AlphabetAddFormDialogComponent,
    );

    dialogRef.afterClosed().subscribe((result) => {
      if (!result) return;
      this.alphabetStore.chooseAlphabet(result).subscribe((alphabetEntity) => {
        this.$getAllTitles();
        const selectHtml = this.selectHtml();
        if (!selectHtml) return;
        selectHtml.value = alphabetEntity.id;
      });
    });
  }

  onDeleteAlphabet() {
    const alphabetId = this.alphabetStore.getCurrentAlphabetId();
    if (!alphabetId) return;

    const currAlphabet = this.alphabetStore.getCurrentAlphabet();
    if (!currAlphabet) return;

    const dialogRef = this.confirmDeleteAlphabetDialog.open(
      ConfirmDeleteAlphabetDialogComponent,
      {
        data: {
          alphabet: currAlphabet,
        },
      },
    );

    dialogRef.afterClosed().subscribe((result) => {
      if (!result) return;
      this.alphabetStore.deleteAlphabet(alphabetId);
    });
  }

  onAddLetter(letter: string) {
    window.location.href = '/date-ideas?letter=' + letter;
  }

  onDeleteIdeaAlphabet(dateIdea: DateIdeaEntity) {
    this.alphabetStore
      .removeDateIdea(dateIdea.id)
      .pipe(
        tap(() => {
          window.location.href = '/';
        }),
      )
      .subscribe();
  }

  onExportAlphabet() {
    const alphabet = this.alphabetStore.getCurrentAlphabet();
    const blob = new Blob([JSON.stringify(alphabet)], {
      type: 'application/json',
    });
    const url = URL.createObjectURL(blob);
    const title = this.alphabetStore.getCurrentAlphabet()?.title;
    const a = document.createElement('a');
    a.href = url;
    a.download = (title ? title : 'alphabet') + '.json';
    a.click();
    URL.revokeObjectURL(url);
    a.remove();
  }

  onChangeCheckboxCompleteDate(
    userDate: UserDateEntity,
    complete: MatCheckbox,
    checkedIcon: MatIcon,
    event: MouseEvent,
  ) {
    complete.disabled = true;

    event.preventDefault();
    event.stopPropagation();
    event.stopImmediatePropagation();

    this.alphabetStore.toggleCompleteDate(userDate.id).subscribe({
      next: (completeRes) => {
        complete.checked = completeRes.completed;
        const element = document.getElementById('panel-' + userDate.letter);
        if (!element) return;
        if (complete.checked) {
          element.classList.add('checked');
          checkedIcon._elementRef.nativeElement.style.opacity = '1';
        } else {
          element.classList.remove('checked');
          checkedIcon._elementRef.nativeElement.style.opacity = '0';
        }
      },
      complete: () => {
        complete.disabled = false;
      },
    });
  }

  onRandomIdea() {
    const alphabetId = this.alphabetStore.getCurrentAlphabetId();
    if (!alphabetId) return;

    const currAlphabet = this.alphabetStore.getCurrentAlphabet();
    if (!currAlphabet) return;

    const dialogRef =
      this.confirmDeleteAlphabetDialog.open(RandomIdeaComponent);

    dialogRef.afterClosed().subscribe((result) => {
      if (!result) return;

      const dateIdea = result.dateIdea;

      this.alphabetStore.addDateIdea(dateIdea.id).subscribe(() => {
        window.location.href = `/?letter=${dateIdea.idea.charAt(0)}`;
      });
    });
  }
}
