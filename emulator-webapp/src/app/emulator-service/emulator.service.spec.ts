import { TestBed } from '@angular/core/testing';

import { EmulatorService } from './emulator.service';

describe('EmulatorService', () => {
  let service: EmulatorService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(EmulatorService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
