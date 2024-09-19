import { ComponentFixture, TestBed } from '@angular/core/testing';

import { IdeaUpdateFormDialogComponent } from './idea-update-form-dialog.component';

describe('IdeaUpdateFormDialogComponent', () => {
  let component: IdeaUpdateFormDialogComponent;
  let fixture: ComponentFixture<IdeaUpdateFormDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [IdeaUpdateFormDialogComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(IdeaUpdateFormDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
