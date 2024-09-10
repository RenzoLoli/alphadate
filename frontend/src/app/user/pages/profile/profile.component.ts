import { Component, effect, inject, OnInit, signal } from '@angular/core';
import {
  FormControl,
  FormGroup,
  ReactiveFormsModule,
  Validators,
} from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import {
  MatFormFieldAppearance,
  MatFormFieldModule,
} from '@angular/material/form-field';
import { MatIconModule } from '@angular/material/icon';
import { MatInputModule } from '@angular/material/input';
import { UserUpdateRequest } from '../../models/user-update.request';
import { AuthStore } from '../../store/auth.store';

const MATERIAL: Array<any> = [
  MatFormFieldModule,
  MatInputModule,
  MatButtonModule,
  MatIconModule,
];
const MODULES: Array<any> = [ReactiveFormsModule];
const COMPONENTS: Array<Component> = [];

@Component({
  selector: 'app-profile',
  standalone: true,
  imports: [MATERIAL, MODULES, COMPONENTS],
  templateUrl: './profile.component.html',
  styleUrl: './profile.component.css',
})
export class ProfileComponent implements OnInit {
  formGroup = new FormGroup({
    username: new FormControl('', {
      validators: [Validators.maxLength(20)],
    }),
    couplename: new FormControl('', {
      validators: [Validators.maxLength(20)],
    }),
    anniversary: new FormControl('', {
      validators: [],
    }),
    photo: new FormControl('', {
      validators: [
        Validators.pattern('^(http|https)://.+.(png|jpg|jpeg|gif)$'),
      ],
    }),
    email: new FormControl('', {
      validators: [Validators.email],
    }),
  });

  editMode = signal<boolean>(false);
  appearance: MatFormFieldAppearance = 'outline';

  authStore = inject(AuthStore);

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

  get email() {
    return this.formGroup.get('email');
  }

  get user() {
    return this.authStore.getUser();
  }

  private editModeChanges = effect(() => {
    const editMode = this.editMode();
    const loading = this.authStore.isLoading();

    this.appearance = editMode ? 'fill' : 'outline';

    if (!loading && !editMode) {
      this.fillForm();
    }
  });

  ngOnInit(): void {
    this.fillForm();
  }

  fillForm() {
    console.log('filled');
    this.formGroup.setValue({
      username: this.user == null ? '' : this.user.username,
      couplename: this.user == null ? '' : this.user.couplename,
      anniversary: this.user == null ? '' : this.user.anniversary,
      photo: this.user == null ? '' : this.user.photo,
      email: this.user == null ? '' : this.user.email,
    });
  }

  onEdit() {
    this.editMode.set(true);
  }

  onSave() {
    const userUpdateRequest: UserUpdateRequest = {
      username: this.username?.value || undefined,
      couplename: this.couplename?.value || undefined,
      anniversary: this.anniversary?.value || undefined,
      photo: this.photo?.value || undefined,
    };
    this.authStore.updateUser(userUpdateRequest);
    this.editMode.set(false);
  }

  onCancel() {
    this.fillForm();
    this.editMode.set(false);
  }
}
