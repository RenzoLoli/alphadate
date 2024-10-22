import { Component, inject, Input, WritableSignal } from '@angular/core';
import { MatChipsModule } from '@angular/material/chips';
import { MatIconModule } from '@angular/material/icon';
import { AlphabetStore } from '../../../alphabet/store/alphabet.store';

const MATERIAL: any[] = [MatChipsModule, MatIconModule];
const COMPONENTS: any[] = [];

@Component({
  selector: 'app-alphabet-letters',
  standalone: true,
  imports: [MATERIAL, COMPONENTS],
  templateUrl: './alphabet-letters.component.html',
  styleUrl: './alphabet-letters.component.css',
})
export class AlphabetLettersComponent {
  alphabetStore = inject(AlphabetStore);

  @Input({ required: true }) letterFilter!: WritableSignal<string>;

  onClickLetter(letter: string) {
    if (this.letterFilter() === letter) letter = '';
    this.letterFilter.set(letter);
  }

  get letters() {
    return this.alphabetStore.getCurrentActiveLetters();
  }
}
