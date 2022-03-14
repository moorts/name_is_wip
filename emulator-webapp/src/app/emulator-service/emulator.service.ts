import { Injectable } from '@angular/core';
import init, { InitOutput } from "emulator";

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
