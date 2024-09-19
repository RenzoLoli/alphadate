import { Component, computed, inject } from '@angular/core';
import { FormControl, Validators, FormGroup } from '@angular/forms';
import { ReactiveFormsModule } from '@angular/forms';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatCardModule } from '@angular/material/card';
import { SignInRequest } from '../../models/sign-in.request';
import { AuthStore } from '../../store/auth.store';

const MATERIAL: Array<any> = [MatInputModule, MatButtonModule, MatCardModule];
const MODULES: Array<any> = [ReactiveFormsModule];
const COMPONENTS: Array<Component> = [];

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [MODULES, MATERIAL, COMPONENTS],
  templateUrl: './login.component.html',
  styleUrl: './login.component.css',
})
export class LoginComponent {
  formGroup = new FormGroup({
    email: new FormControl('test@test.com', {
      nonNullable: true,
      validators: [Validators.required, Validators.email],
    }),
    password: new FormControl('test123', {
      nonNullable: true,
      validators: [Validators.required, Validators.maxLength(20)],
    }),
  });
  authStore = inject(AuthStore);

  get reqError() {
    return this.authStore.getError();
  }

  get email() {
    return this.formGroup.get('email');
  }

  get password() {
    return this.formGroup.get('password');
  }

  onSubmit() {
    if (this.formGroup.invalid) return;

    const signinRequest: SignInRequest = this.formGroup.getRawValue();

    this.authStore.login(signinRequest);
  }
}
