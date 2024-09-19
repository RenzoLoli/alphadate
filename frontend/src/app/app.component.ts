import { Component, inject } from '@angular/core';
import { RouterOutlet } from '@angular/router';

import { CommonModule } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatMenuModule } from '@angular/material/menu';
import { MatSidenavModule } from '@angular/material/sidenav';
import { MatToolbarModule } from '@angular/material/toolbar';
import { SidebarComponent } from './shared/components/sidebar/sidebar.component';
import { ToolbarComponent } from './shared/components/toolbar/toolbar.component';
import { BrandService } from './shared/services/brand.service';
import { UserTagComponent } from './user/components/user-tag/user-tag.component';
import { AuthStore } from './user/store/auth.store';

const COMPONENTS: Array<any> = [
  UserTagComponent,
  ToolbarComponent,
  SidebarComponent,
];
const MATERIAL = [
  MatToolbarModule,
  MatIconModule,
  MatButtonModule,
  MatMenuModule,
  MatSidenavModule,
];

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet, CommonModule, COMPONENTS, MATERIAL],
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
}
