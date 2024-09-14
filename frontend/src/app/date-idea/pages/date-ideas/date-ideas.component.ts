import { CommonModule } from '@angular/common';
import { Component, computed, inject, OnInit, signal } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { rxMethod } from '@ngrx/signals/rxjs-interop';
import { pipe, switchMap, tap } from 'rxjs';
import { FilterComponent } from '../../components/filter/filter.component';
import { IdeasTableComponent } from '../../components/ideas-table/ideas-table.component';
import { DateIdeaEntity } from '../../models/date-idea.entity';
import { TagEntity } from '../../models/tag.entity';
import { DateIdeaService } from '../../services/date-idea.service';
import { TagService } from '../../services/tag.service';
import { MatDialog } from '@angular/material/dialog';
import { IdeaAddFormDialogComponent } from '../../components/idea-add-form-dialog/idea-add-form-dialog.component';
import { IdeaUpdateFormDialogComponent } from '../../components/idea-update-form-dialog/idea-update-form-dialog.component';

const COMPONENTS: Array<any> = [IdeasTableComponent, FilterComponent];
const MATERIAL: Array<any> = [MatButtonModule, MatIconModule];

@Component({
  selector: 'app-date-ideas',
  standalone: true,
  imports: [CommonModule, COMPONENTS, MATERIAL],
  templateUrl: './date-ideas.component.html',
  styleUrl: './date-ideas.component.css',
})
export class DateIdeasComponent implements OnInit {
  dateIdeaService = inject(DateIdeaService);
  tagService = inject(TagService);

  addIdeaDialog = inject(MatDialog);
  updateIdeaDialog = inject(MatDialog);

  dateIdeas = signal(Array<DateIdeaEntity>());
  tags = signal(Array<TagEntity>());

  filterValue = signal('');
  filterTags = signal(Array<TagEntity>());
  filteredDateIdeas = computed(() => this.filterDateIdeas());

  allTags$ = rxMethod<void>(
    pipe(
      switchMap(() => this.tagService.getAll()),
      tap((tags: Array<TagEntity>) => {
        this.tags.set(tags);
      }),
    ),
  );
  allDateIdeas$ = rxMethod<void>(
    pipe(
      switchMap(() => this.dateIdeaService.getAll()),
      tap((dateIdeas) => {
        this.dateIdeas.set(dateIdeas);
      }),
    ),
  );

  ngOnInit(): void {
    this.allDateIdeas$();
    this.allTags$();
  }

  get isAdmin() {
    return true;
  }

  filterDateIdeas = () => {
    const filterValue = this.filterValue();
    const filterTags = this.filterTags();
    return this.dateIdeas().filter((idea) => {
      const isIncludedOnIdea =
        filterValue.length === 0 || idea.idea.includes(filterValue);
      const isIncludedOnDescription =
        filterValue.length === 0 || idea.description.includes(filterValue);
      const hasTags =
        filterTags.length === 0 ||
        filterTags.every((tag) =>
          idea.tags.some((filterTag) => tag.name === filterTag.name),
        );
      return (isIncludedOnIdea || isIncludedOnDescription) && hasTags;
    });
  };

  onRemoveIdeaTag({
    dateIdea,
    tag,
  }: {
    dateIdea: DateIdeaEntity;
    tag: TagEntity;
  }) {}

  onAddIdeaTag({ dateIdea }: { dateIdea: DateIdeaEntity }) {}

  onEditIdea({ dateIdea }: { dateIdea: DateIdeaEntity }) {
    const dialogRef = this.updateIdeaDialog.open(
      IdeaUpdateFormDialogComponent,
      {
        data: dateIdea,
      },
    );
    dialogRef.afterClosed().subscribe((result) => {
      if (!result) return;
      this.allDateIdeas$();
    });
  }

  onDeleteIdea({ dateIdea }: { dateIdea: DateIdeaEntity }) {}

  onAddIdeaAlphabet({ dateIdea }: { dateIdea: DateIdeaEntity }) {}

  onAddIdea() {
    const dialogRef = this.addIdeaDialog.open(IdeaAddFormDialogComponent);

    dialogRef.afterClosed().subscribe((result) => {
      if (!result) return;
      this.allDateIdeas$();
    });
  }
}
