import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RandomDateIdeaDialogComponent } from './random-date-idea-dialog.component';

describe('RandomDateIdeaDialogComponent', () => {
  let component: RandomDateIdeaDialogComponent;
  let fixture: ComponentFixture<RandomDateIdeaDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [RandomDateIdeaDialogComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(RandomDateIdeaDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
