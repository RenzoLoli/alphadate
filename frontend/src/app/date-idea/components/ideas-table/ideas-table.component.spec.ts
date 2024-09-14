import { ComponentFixture, TestBed } from '@angular/core/testing';

import { IdeasTableComponent } from './ideas-table.component';

describe('IdeasTableComponent', () => {
  let component: IdeasTableComponent;
  let fixture: ComponentFixture<IdeasTableComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [IdeasTableComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(IdeasTableComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
