import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DateIdeasComponent } from './date-ideas.component';

describe('DateIdeasComponent', () => {
  let component: DateIdeasComponent;
  let fixture: ComponentFixture<DateIdeasComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [DateIdeasComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DateIdeasComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
