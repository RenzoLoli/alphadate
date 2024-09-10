import { Component, inject } from '@angular/core';
import { RouterOutlet } from '@angular/router';

import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatMenuModule } from '@angular/material/menu';
import { MatToolbarModule } from '@angular/material/toolbar';
import { BrandService } from './shared/services/brand.service';
import { UserTagComponent } from './user/components/user-tag/user-tag.component';
import { AuthStore } from './user/store/auth.store';

const COMPONENTS: Array<any> = [UserTagComponent];
const MATERIAL = [
  MatToolbarModule,
  MatIconModule,
  MatButtonModule,
  MatMenuModule,
];

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet, COMPONENTS, MATERIAL],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css',
})
export class AppComponent {
  authStore = inject(AuthStore);

  brandService = inject(BrandService);

  get user() {
    return this.authStore.getUser();
  }

  get isAuthenticated() {
    return this.authStore.isAuthenticated();
  }

  logout() {
    this.authStore.logout();
  }
}
