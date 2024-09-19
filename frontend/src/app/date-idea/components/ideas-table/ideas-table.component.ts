import {
  Component,
  computed,
  effect,
  Input,
  OnInit,
  output,
  Signal,
} from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatChipsModule } from '@angular/material/chips';
import { MatIconModule } from '@angular/material/icon';
import { MatMenuModule } from '@angular/material/menu';
import { MatTableModule } from '@angular/material/table';
import { DateIdeaEntity } from '../../models/date-idea.entity';
import { TagEntity } from '../../models/tag.entity';

const COMPONENTS: Array<any> = [];
const MATERIAL: Array<any> = [
  MatTableModule,
  MatChipsModule,
  MatIconModule,
  MatButtonModule,
  MatButtonModule,
  MatMenuModule,
];

@Component({
  selector: 'app-ideas-table',
  standalone: true,
  imports: [COMPONENTS, MATERIAL],
  templateUrl: './ideas-table.component.html',
  styleUrl: './ideas-table.component.css',
})
export class IdeasTableComponent implements OnInit {
  @Input({ required: true }) dateIdeas!: Signal<Array<DateIdeaEntity>>;
  @Input({ required: true }) tags!: Signal<Array<TagEntity>>;
  @Input({ required: true }) admin!: boolean;

  displayedColumns: string[] = ['idea', 'description', 'tags'];

  ngOnInit(): void {
    if (this.admin) {
      this.displayedColumns.push('actions');
    }
  }

  missingTags = (dateIdea: DateIdeaEntity) =>
    this.tags().filter((tag) => {
      return !dateIdea.tags.some((dateIdeaTag) => tag.id === dateIdeaTag.id);
    });

  removeIdeaTag = output<{ dateIdea: DateIdeaEntity; tag: TagEntity }>();
  addIdeaTag = output<{ dateIdea: DateIdeaEntity; tag: TagEntity }>();
  editIdea = output<{ dateIdea: DateIdeaEntity }>();
  deleteIdea = output<{ dateIdea: DateIdeaEntity }>();
  addIdeaAlphabet = output<{ dateIdea: DateIdeaEntity }>();

  onRemoveIdeaTag(dateIdea: DateIdeaEntity, tag: TagEntity) {
    this.removeIdeaTag.emit({ dateIdea, tag });
  }

  onAddIdeaTag(dateIdea: DateIdeaEntity, tag: TagEntity) {
    this.addIdeaTag.emit({ dateIdea, tag });
  }

  onEditIdea(dateIdea: DateIdeaEntity) {
    this.editIdea.emit({ dateIdea });
  }

  onDeleteIdea(dateIdea: DateIdeaEntity) {
    this.deleteIdea.emit({ dateIdea });
  }

  onAddIdeaAlphabet(dateIdea: DateIdeaEntity) {
    this.addIdeaAlphabet.emit({ dateIdea });
  }
}
