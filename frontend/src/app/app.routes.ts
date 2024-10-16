import { Routes } from '@angular/router';
import { DateIdeasComponent } from './date-idea/pages/date-ideas/date-ideas.component';
import AuthGuard from './user/guards/auth.guard';
import { LoginComponent } from './user/pages/login/login.component';
import { ProfileComponent } from './user/pages/profile/profile.component';
import { RegisterComponent } from './user/pages/register/register.component';
import { HomeComponent } from './alphabet/pages/home/home.component';
import { LogoutComponent } from './user/pages/logout/logout.component';

export const routes: Routes = [
  {
    path: '',
    children: [
      {
        path: '',
        title: 'Alphadate',
        component: HomeComponent,
      },
      {
        path: 'logout',
        title: 'logout',
        component: LogoutComponent,
      },
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
        component: DateIdeasComponent,
      },
      {
        path: '**',
        redirectTo: '',
        pathMatch: 'full',
      },
    ],
    canActivate: [AuthGuard],
    runGuardsAndResolvers: 'always',
  },
];
