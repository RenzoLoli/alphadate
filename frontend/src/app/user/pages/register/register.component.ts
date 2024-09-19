import { Component, computed, inject } from '@angular/core';
import {
  FormControl,
  FormGroup,
  ReactiveFormsModule,
  Validators,
} from '@angular/forms';
import { AuthStore } from '../../store/auth.store';
import { SignUpRequest } from '../../models/sign-up.request';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';

const MATERIAL: Array<any> = [
  MatFormFieldModule,
  MatInputModule,
  MatButtonModule,
  MatCardModule,
];
const MODULES: Array<any> = [ReactiveFormsModule];
const COMPONENTS: Array<Component> = [];

@Component({
  selector: 'app-register',
  standalone: true,
  imports: [MODULES, MATERIAL, COMPONENTS],
  templateUrl: './register.component.html',
  styleUrl: './register.component.css',
})
export class RegisterComponent {
  formGroup = new FormGroup({
    username: new FormControl('test', {
      nonNullable: true,
      validators: [Validators.required, Validators.maxLength(20)],
    }),
    couplename: new FormControl('testa', {
      nonNullable: true,
      validators: [Validators.required, Validators.maxLength(20)],
    }),
    anniversary: new FormControl('2020-02-03', {
      nonNullable: true,
      validators: [Validators.required],
    }),
    photo: new FormControl(
      'https://img.freepik.com/vector-gratis/ilustracion-pareja-asiatica-dibujada-mano_23-2150016652.jpg',
      {
        nonNullable: true,
        validators: [
          Validators.required,
          Validators.pattern('^(http|https)://.+.(png|jpg|jpeg|gif)$'),
        ],
      },
    ),
    email: new FormControl('test@test.com', {
      nonNullable: true,
      validators: [Validators.required, Validators.email],
    }),
    password: new FormControl('test123', {
      nonNullable: true,
      validators: [Validators.required, Validators.minLength(6)],
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

  get username() {
    return this.formGroup.get('username');
  }

  get couplename() {
    return this.formGroup.get('couplename');
  }

  get anniversary() {
    return this.formGroup.get('anniversary');
  }

  get photo() {
    return this.formGroup.get('photo');
  }

  onSubmit() {
    if (this.formGroup.invalid) return;

    const signupRequest: SignUpRequest = this.formGroup.getRawValue();

    this.authStore.register(signupRequest);
  }
}
