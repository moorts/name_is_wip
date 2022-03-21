import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RamDisplayComponent } from './ram-display.component';

describe('RamDisplayComponent', () => {
  let component: RamDisplayComponent;
  let fixture: ComponentFixture<RamDisplayComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ RamDisplayComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(RamDisplayComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
