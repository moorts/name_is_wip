import { Injectable } from '@angular/core';
import init, { assemble } from "emulator";

@Injectable({
  providedIn: 'root'
})
export class EmulatorService {

  constructor() {
    this.initialize();
  }

  async initialize() {
    await init("assets/emulator/emulator_bg.wasm");
  }

  public assemble(assembly: string) {
    console.log("Assembling: " + assembly);
    const result = assemble(assembly);
    console.log(result);
  }
}
