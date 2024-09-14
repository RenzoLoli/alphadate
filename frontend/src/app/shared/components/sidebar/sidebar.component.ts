import { Component, inject, output } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatMenuModule } from '@angular/material/menu';
import { UserTagComponent } from '../../../user/components/user-tag/user-tag.component';
import { AuthStore } from '../../../user/store/auth.store';
import { BrandService } from '../../services/brand.service';

const COMPONENTS: Array<any> = [UserTagComponent];
const MATERIAL: Array<any> = [MatIconModule, MatButtonModule, MatMenuModule];

@Component({
  selector: 'app-sidebar',
  standalone: true,
  imports: [COMPONENTS, MATERIAL],
  templateUrl: './sidebar.component.html',
  styleUrl: './sidebar.component.css',
})
export class SidebarComponent {
  authStore = inject(AuthStore);
  brandService = inject(BrandService);

  toggleDrawer = output<void>();

  get isAuthenticated() {
    return this.authStore.isAuthenticated();
  }

  get username() {
    return this.authStore.getUser()?.username;
  }

  get photo() {
    return this.authStore.getUser()?.photo;
  }

  get brandImage() {
    return this.brandService.getBrandImage();
  }

  onToggleDrawer() {
    this.toggleDrawer.emit();
  }
}
