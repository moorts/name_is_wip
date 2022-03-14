import { Injectable } from '@angular/core';
import init from "emulator";
import { InitOutput } from '../../../dist/emulator-webapp/assets/emulator/emulator';

@Injectable({
  providedIn: 'root'
})
export class EmulatorService {

  private emulator: InitOutput | undefined;

  constructor() {
    this.initialize();
  }

  async initialize() {
    this.emulator = await init("assets/emulator/emulator_bg.wasm");
  }

  public assemble(assembly: string) {
    console.log("Assembling: " + assembly);
  }
}
