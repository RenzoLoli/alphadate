import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ConfirmDeleteAlphabetDialogComponent } from './confirm-delete-alphabet-dialog.component';

describe('ConfirmDeleteAlphabetDialogComponent', () => {
  let component: ConfirmDeleteAlphabetDialogComponent;
  let fixture: ComponentFixture<ConfirmDeleteAlphabetDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ConfirmDeleteAlphabetDialogComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ConfirmDeleteAlphabetDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
