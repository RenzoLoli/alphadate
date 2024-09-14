import { Component, inject, output } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatMenuModule } from '@angular/material/menu';
import { MatToolbarModule } from '@angular/material/toolbar';
import { UserTagComponent } from '../../../user/components/user-tag/user-tag.component';
import { AuthStore } from '../../../user/store/auth.store';
import { BrandService } from '../../services/brand.service';

const MATERIAL: Array<any> = [
  MatToolbarModule,
  MatMenuModule,
  MatIconModule,
  MatButtonModule,
];
const COMPONENTS: Array<any> = [UserTagComponent];

@Component({
  selector: 'app-toolbar',
  standalone: true,
  imports: [MATERIAL, COMPONENTS],
  templateUrl: './toolbar.component.html',
  styleUrl: './toolbar.component.css',
})
export class ToolbarComponent {
  authStore = inject(AuthStore);

  brandService = inject(BrandService);

  get isAuthenticated() {
    return this.authStore.isAuthenticated();
  }

  get user() {
    return this.authStore.getUser();
  }

  get brandImage() {
    return this.brandService.getBrandImage();
  }

  toggleDrawer = output<void>();

  onToggleDrawer() {
    this.toggleDrawer.emit();
  }
}
