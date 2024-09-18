import {
  Component,
  Input,
  signal,
  Signal,
  WritableSignal,
} from '@angular/core';
import {
  MatAutocompleteModule,
  MatAutocompleteSelectedEvent,
} from '@angular/material/autocomplete';
import { MatButtonModule } from '@angular/material/button';
import { MatChipEvent, MatChipsModule } from '@angular/material/chips';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatIconModule } from '@angular/material/icon';
import { MatInputModule } from '@angular/material/input';
import { TagEntity } from '../../models/tag.entity';

const COMPONENTS: Array<any> = [];
const MATERIAL: Array<any> = [
  MatFormFieldModule,
  MatInputModule,
  MatChipsModule,
  MatIconModule,
  MatAutocompleteModule,
  MatButtonModule,
];

@Component({
  selector: 'app-filter',
  standalone: true,
  imports: [COMPONENTS, MATERIAL],
  templateUrl: './filter.component.html',
  styleUrl: './filter.component.css',
})
export class FilterComponent {
  @Input({ required: true }) filterValue!: WritableSignal<string>;
  @Input({ required: true }) filterTags!: WritableSignal<Array<TagEntity>>;
  @Input({ required: true }) admin!: boolean;
  @Input({ required: true }) tags!: Signal<Array<TagEntity>>;
  tagInput = signal('');

  get unselectedTags(): Array<TagEntity> {
    return this.tags().filter(
      (t) => !this.filterTags().includes(t) && t.name.includes(this.tagInput()),
    );
  }

  get isAdmin(): boolean {
    return this.admin;
  }

  onChangeFilterIdea(event: KeyboardEvent) {
    const { value } = event.target as HTMLInputElement;
    this.filterValue.set(value);
  }

  onRemoveFilterTag(event: MatChipEvent, tag: TagEntity) {
    this.filterTags.update((tags) => tags.filter((t) => t !== tag));
  }

  onSelectFilterTag(event: MatAutocompleteSelectedEvent) {
    this.filterTags.update((tags) => [...tags, event.option.value]);
    event.option.deselect();
  }

  onChangeFilterTag(event: KeyboardEvent) {
    const { value } = event.target as HTMLInputElement;
    this.tagInput.set(value);
  }
}
