import { Injectable, EventEmitter } from '@angular/core';
import init, { assemble, createEmulator, Emulator, InitOutput } from "emulator";

@Injectable({
  providedIn: 'root'
})
export class EmulatorService {

  public onStep: EventEmitter<any> = new EventEmitter();

  private _wasmContext: InitOutput | undefined;
  private _initialMemory: Uint8Array = new Uint8Array();
  private _running: boolean = false;
  private _paused: boolean = false;
  private _emulator: Emulator | undefined;
  private _emulatorMemory: Uint8Array = new Uint8Array();
  private _step: number = 0;
  private _loop: number = 0;

  // Total speed: _stepsPerInterval * (1000 / _interval) instructions per second
  private _interval: number = 10; // Interval in milliseconds
  private _stepsPerInterval: number = 1; // How many CPU steps to perform per interval (-> allows emulator to perform faster than 1000 i/s)
  private _skipOnStepInterval: number = 1; // OnStep (updates UI for RAM etc.) will only be executed every _skipOnStepInterval steps to improve performance

  public get memory(): Uint8Array {
    if (this._running && this._wasmContext) {
      return this._emulatorMemory;
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

  public get step() {
    return this._step;
  }

  public get emulator() {
    return this._emulator;
  }

  public get registers() {
    if (!this._emulator || !this._wasmContext) return null;
    const ptr = (<any>this._emulator.reg).ptr;
    const memory = new Uint8Array(this._wasmContext.memory.buffer, ptr);

    return {
      b: memory[7],
      c: memory[6],
      d: memory[9],
      e: memory[8],
      h: memory[11],
      l: memory[10],
      a: memory[12]
    };
  }

  public get largeRegisters() {
    if (!this._emulator || !this._wasmContext) return null;
    const ptr = (<any>this._emulator.reg).ptr;
    const memory = new Uint16Array(this._wasmContext.memory.buffer, ptr);

    return {
      b: memory[3],
      d: memory[4],
      h: memory[5],
      psw: memory[6],
      sp: this._emulator.sp
    };
  }

  constructor() {
    this.initialize();
  }

  async initialize() {
    this._wasmContext = await init("assets/emulator/emulator_bg.wasm");
  }

  public loadBytes(buffer: ArrayBuffer) {
    this._initialMemory = new Uint8Array(buffer);
  }

  public assemble(assembly: string) {
    console.log("Assembling: " + assembly);
    const result = assemble(assembly);
    this._initialMemory = result;
  }

  public start() {
    this._emulator = createEmulator(this._initialMemory);
    if (this._wasmContext)
      this._emulatorMemory = new Uint8Array(this._wasmContext.memory.buffer, this._emulator?.get_ram_ptr());
    this._step = 0;
    this._running = true;
    if (!this._paused) {
      this.startLoop();
    }
  }

  public stop() {
    if (!this._running) return;

    this._running = false;
    this._emulator = undefined;
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

  public doStep() {
    if (!this.running || !this.paused || this.halted) return;
    this.cpuStep();
  }

  private startLoop() {
    this._loop = window.setInterval(() => {
      for (let i = 0; i < this._stepsPerInterval; i++) {
        this.cpuStep();
      }
    }, this._interval);
  }

  private cpuStep() {
    if (!this._emulator) return;

    const prevMemAddress = this._emulator.get_last_ram_change();
    const prevMem = this.memory[prevMemAddress];

    this._emulator.execute_next();
    this._step += 1;

    const newMemAddress = this._emulator.get_last_ram_change();
    const ramChanged = prevMemAddress != newMemAddress || this.memory[prevMemAddress] != prevMem;

    if (this._step % this._skipOnStepInterval == 0) {
      this.onStep.emit({
        ramChanged: ramChanged
      });
    }

    if (!this._emulator.running) {
      console.log("CPU halted");
      window.clearInterval(this._loop);
    }
  }
}
