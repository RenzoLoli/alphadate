import { Component, inject } from '@angular/core';
import { Router } from '@angular/router';
import { AuthStore } from '../../store/auth.store';
import { AlphabetStore } from '../../../alphabet/store/alphabet.store';

@Component({
  selector: 'app-logout',
  standalone: true,
  imports: [],
  templateUrl: './logout.component.html',
  styleUrl: './logout.component.css',
})
export class LogoutComponent {
  authStore = inject(AuthStore);
  alphabetStore = inject(AlphabetStore);
  constructor(private router: Router) {
    this.authStore.logout();
    this.alphabetStore.clear();
    this.router.navigate(['/login']);
  }
}
