import { Component, computed, inject, OnInit, signal } from '@angular/core';
import { ReactiveFormsModule } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import { MatChipsModule } from '@angular/material/chips';
import { MatDialogModule, MatDialogRef } from '@angular/material/dialog';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatIconModule } from '@angular/material/icon';
import { MatInputModule } from '@angular/material/input';
import { rxMethod } from '@ngrx/signals/rxjs-interop';
import { catchError, pipe, switchMap, tap } from 'rxjs';
import { TagCreateRequest } from '../../models/tag-create.request';
import { TagDeleteRequest } from '../../models/tag-delete.request';
import { TagEntity } from '../../models/tag.entity';
import { TagService } from '../../services/tag.service';

const COMPONENTS: Array<any> = [];
const MATERIAL: Array<any> = [
  MatDialogModule,
  MatFormFieldModule,
  MatInputModule,
  MatButtonModule,
  MatIconModule,
  MatChipsModule,
];

@Component({
  selector: 'app-tag-editor-dialog',
  standalone: true,
  imports: [ReactiveFormsModule, COMPONENTS, MATERIAL],
  templateUrl: './tag-editor-dialog.component.html',
  styleUrl: './tag-editor-dialog.component.css',
})
export class TagEditorDialogComponent implements OnInit {
  readonly dialogRef = inject(MatDialogRef<TagEditorDialogComponent>);
  tagService = inject(TagService);

  tags = signal(Array<TagEntity>());
  tagInput = signal('');
  filteredTags = computed(() => {
    return this.tags().filter((tag: TagEntity) => {
      return tag.name.toLowerCase().includes(this.tagInput().toLowerCase());
    });
  });

  allTags$ = rxMethod<void>(
    pipe(
      switchMap(() => this.tagService.getAll()),
      tap((tags: Array<TagEntity>) => {
        this.reqError.set('');
        this.tags.set(tags);
      }),
    ),
  );

  createTag = rxMethod<TagCreateRequest>(
    pipe(
      switchMap((value) => {
        return this.tagService.create(value);
      }),
      tap(() => {
        this.allTags$();
        this.tagInput.set('');
      }),
      catchError((error, caugth) => {
        this.reqError.set(error.message);
        return caugth;
      }),
    ),
  );

  removeTag = rxMethod<TagDeleteRequest>(
    pipe(
      switchMap((request) => {
        return this.tagService.delete(request.id);
      }),
      tap(() => {
        this.allTags$();
      }),
      catchError((error, caught) => {
        this.reqError.set(error.message);
        return caught;
      }),
    ),
  );

  reqError = signal('');

  ngOnInit(): void {
    this.allTags$();
  }

  onExit() {
    this.dialogRef.close();
  }

  onAddTag(input: HTMLInputElement) {
    if (this.tagInput() === '') return;

    const createTagRequest: TagCreateRequest = {
      name: this.tagInput(),
    };

    this.createTag(createTagRequest);
    input.value = '';
  }

  onRemoveTag(tag: TagEntity) {
    const deleteTagRequest: TagDeleteRequest = {
      id: tag.id,
    };

    this.removeTag(deleteTagRequest);
  }

  onKeyUp(event: KeyboardEvent) {
    const element = event.target as HTMLInputElement;
    this.tagInput.set(element.value);
  }
}
