import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RightComponent } from './right.component';

describe('RightComponent', () => {
  let component: RightComponent;
  let fixture: ComponentFixture<RightComponent>;

  beforeEach(() => {
    TestBed.configureTestingModule({
      declarations: [RightComponent]
    });
    fixture = TestBed.createComponent(RightComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
