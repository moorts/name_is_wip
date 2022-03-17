import { Injectable } from '@angular/core';
import init, { assemble, createEmulator, Emulator, InitOutput } from "emulator";

@Injectable({
  providedIn: 'root'
})
export class EmulatorService {

  private _wasmContext: InitOutput | undefined;
  private _initialMemory: Uint8Array = new Uint8Array();
  private _running: boolean = false;
  private _paused: boolean = false;
  private _emulator: Emulator | undefined;
  private _step: number = 0;
  private _loop: number = 0;
  private _interval: number = 100;

  public get memory(): Uint8Array {
    if (this._running) {
      // TODO: return emulator memory
      return this._initialMemory;
    } else {
      return this._initialMemory;
    }
  }

  public get running() {
    return this._running;
  }

  public get paused() {
    return this._paused;
  }

  public get halted() {
    return !this._emulator?.running;
  }

  constructor() {
    this.initialize();
  }

  async initialize() {
    this._wasmContext = await init("assets/emulator/emulator_bg.wasm");
  }

  public assemble(assembly: string) {
    console.log("Assembling: " + assembly);
    const result = assemble(assembly);
    this._initialMemory = result;
  }

  public start() {
    this._emulator = createEmulator(this._initialMemory);
    this._step = 0;
    this._running = true;
    if (!this._paused) {
      this.startLoop();
    }
  }

  public stop() {
    if (!this._running) return;

    this._running = false;
    window.clearInterval(this._loop);
  }

  public togglePause() {
    this._paused = !this._paused;
    if (this._running) {
      if (this.paused) {
        window.clearInterval(this._loop);
      } else {
        this.startLoop();
      }
    }
  }

  public step() {
    if (!this.running || !this.paused || this.halted) return;
    this.cpuStep();
  }

  private startLoop() {
    this._loop = window.setInterval(() => this.cpuStep(), this._interval);
  }

  private cpuStep() {
    if (!this._emulator) return;

    this._emulator.execute_next();
    this.logEmulatorStatus();
    this._step += 1;

    if (!this._emulator.running) {
      console.log("CPU halted");
      //this.logEmulatorStatus();
      window.clearInterval(this._loop);
    }
  }

  private logEmulatorStatus() {
    if (!this._emulator || !this._wasmContext) return;
    console.log("Emulator Step: " + this._step);
    const ptr = (<any>this._emulator.reg).ptr;
    const memory = new Uint8Array(this._wasmContext.memory.buffer);

    const reg_b = memory[ptr + 7];
    const reg_c = memory[ptr + 6];
    const reg_d = memory[ptr + 9];
    const reg_e = memory[ptr + 8];
    const reg_h = memory[ptr + 11];
    const reg_l = memory[ptr + 10];
    const reg_a = memory[ptr + 12];

    console.log("B: " + reg_b);
    console.log("C: " + reg_c);
    console.log("D: " + reg_d);
    console.log("E: " + reg_e);
    console.log("H: " + reg_h);
    console.log("L: " + reg_l);
    console.log("A: " + reg_a);
  }
}
