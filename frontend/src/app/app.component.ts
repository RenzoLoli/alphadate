import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { MatSlideToggleModule } from '@angular/material/slide-toggle';

const materialImports = [MatSlideToggleModule];
const routerImports = [RouterOutlet];

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [...materialImports, ...routerImports],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css',
})
export class AppComponent {}
