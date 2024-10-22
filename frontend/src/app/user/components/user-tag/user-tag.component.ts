import { Component, inject, Input } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatCardModule } from '@angular/material/card';
import { MatIconModule } from '@angular/material/icon';
import { MatMenuModule } from '@angular/material/menu';
import { AuthStore } from '../../store/auth.store';

const COMPONENTS = [
  MatCardModule,
  MatButtonModule,
  MatMenuModule,
  MatIconModule,
];

@Component({
  selector: 'app-user-tag',
  standalone: true,
  imports: [COMPONENTS],
  templateUrl: './user-tag.component.html',
  styleUrl: './user-tag.component.css',
})
export class UserTagComponent {
  authStore = inject(AuthStore);

  @Input() minimal: boolean = false;

  get alias() {
    return this.authStore.getUser()?.username || '';
  }

  get photo() {
    return this.authStore.getUser()?.photo || '';
  }

  logout() {
    window.location.href = '/logout';
  }
}
