import { Routes } from '@angular/router';
import { LoginComponent } from './user/pages/login/login.component';
import { RegisterComponent } from './user/pages/register/register.component';
import AuthGuard from './user/guards/auth.guard';
import { ProfileComponent } from './user/pages/profile/profile.component';
import { DateIdeasComponent } from './date-idea/pages/date-ideas/date-ideas.component';

export const routes: Routes = [
  {
    path: '',
    children: [
      {
        path: 'login',
        title: 'Login',
        component: LoginComponent,
      },
      {
        path: 'register',
        title: 'Register',
        component: RegisterComponent,
      },
      {
        path: 'profile',
        title: 'Profile',
        component: ProfileComponent,
      },
      {
        path: 'date-ideas',
        title: 'Date Ideas',
        component: DateIdeasComponent
      }
    ],
    canActivate: [AuthGuard],
    runGuardsAndResolvers: 'always',
  },
];
