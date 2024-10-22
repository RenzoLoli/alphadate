import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AlphabetLettersComponent } from './alphabet-letters.component';

describe('AlphabetLettersComponent', () => {
  let component: AlphabetLettersComponent;
  let fixture: ComponentFixture<AlphabetLettersComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AlphabetLettersComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AlphabetLettersComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
