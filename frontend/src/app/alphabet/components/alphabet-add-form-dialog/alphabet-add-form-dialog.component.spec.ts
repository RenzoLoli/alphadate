import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AlphabetAddFormDialogComponent } from './alphabet-add-form-dialog.component';

describe('AlphabetAddFormDialogComponent', () => {
  let component: AlphabetAddFormDialogComponent;
  let fixture: ComponentFixture<AlphabetAddFormDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AlphabetAddFormDialogComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AlphabetAddFormDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
