import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RandomIdeaComponent } from './random-idea.component';

describe('RandomIdeaComponent', () => {
  let component: RandomIdeaComponent;
  let fixture: ComponentFixture<RandomIdeaComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [RandomIdeaComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(RandomIdeaComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
