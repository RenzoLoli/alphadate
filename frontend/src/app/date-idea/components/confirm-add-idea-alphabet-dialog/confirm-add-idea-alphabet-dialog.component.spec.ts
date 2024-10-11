import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ConfirmAddIdeaAlphabetDialogComponent } from './confirm-add-idea-alphabet-dialog.component';

describe('ConfirmAddIdeaAlphabetDialogComponent', () => {
  let component: ConfirmAddIdeaAlphabetDialogComponent;
  let fixture: ComponentFixture<ConfirmAddIdeaAlphabetDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ConfirmAddIdeaAlphabetDialogComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ConfirmAddIdeaAlphabetDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
