import { ComponentFixture, TestBed } from '@angular/core/testing';

import { IdeaAddFormDialogComponent } from './idea-add-form-dialog.component';

describe('IdeaAddFormDialogComponent', () => {
  let component: IdeaAddFormDialogComponent;
  let fixture: ComponentFixture<IdeaAddFormDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [IdeaAddFormDialogComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(IdeaAddFormDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
