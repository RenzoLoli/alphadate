import { Component, Input } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatCardModule } from '@angular/material/card';

const COMPONENTS = [MatCardModule, MatButtonModule];

@Component({
  selector: 'app-user-tag',
  standalone: true,
  imports: [COMPONENTS],
  templateUrl: './user-tag.component.html',
  styleUrl: './user-tag.component.css',
})
export class UserTagComponent {
  @Input() img: string = '';
  @Input() alias?: string;
}
