import { ComponentFixture, TestBed } from '@angular/core/testing';

import { LoadFileDialogComponent } from './load-file-dialog.component';

describe('LoadFileDialogComponent', () => {
  let component: LoadFileDialogComponent;
  let fixture: ComponentFixture<LoadFileDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ LoadFileDialogComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(LoadFileDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
