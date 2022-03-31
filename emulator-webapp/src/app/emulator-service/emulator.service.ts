import { Injectable, EventEmitter } from '@angular/core';
import init, { assemble, createEmulator, disassemble, Emulator, InitOutput, registerSpaceInvadersDevices } from "emulator";

@Injectable({
  providedIn: 'root'
})
export class EmulatorService {

  public onStep: EventEmitter<any> = new EventEmitter();
  public onVideoStep: EventEmitter<any> = new EventEmitter();

  public _wasmContext: InitOutput | undefined;
  private _initialMemory: Uint8Array = new Uint8Array();
  private _running: boolean = false;
  private _paused: boolean = false;
  private _emulator: Emulator | undefined;
  private _emulatorMemory: Uint8Array = new Uint8Array();
  private _step: number = 0;
  private _loop: number = 0;
  private _videoloop: number = 0;

  private _cpmMode: boolean = false;

  // Total speed: _stepsPerInterval * (1000 / _interval) instructions per second
  private _interval: number = 1; // Interval in milliseconds
  private _stepsPerInterval: number = 1000; // How many CPU steps to perform per interval (-> allows emulator to perform faster than 1000 i/s)
  private _skipOnStepInterval: number = 1000000; // OnStep (updates UI for RAM etc.) will only be executed every _skipOnStepInterval steps to improve performance

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

  public disassemble(bytes: Uint8Array): string {
    const result = disassemble(bytes);
    return result;
  }

  public start() {
    this._emulator = createEmulator(this._initialMemory);
    registerSpaceInvadersDevices(this._emulator);
    if (this._wasmContext)
      this._emulatorMemory = new Uint8Array(this._wasmContext.memory.buffer, this._emulator?.get_ram_ptr(), 0x4000);
    this._step = 0;
    this._running = true;

    if (this._cpmMode) {
      // Patch memory for BDOS syscalls
      this._emulatorMemory[5] = 0xC9; // RET
      this._emulator.pc = 0x100;
    }

    if (!this._paused) {
      this.startLoop();
    }
  }

  public stop() {
    if (!this._running) return;

    this._running = false;
    this._emulator = undefined;
    this.stopLoop();
  }

  public togglePause() {
    this._paused = !this._paused;
    if (this._running) {
      if (this.paused) {
        this.stopLoop();
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
    this._videoloop = window.setInterval(() => {
      this.videoStep();
    }, 1000/60);
  }

  private stopLoop() {
    window.clearInterval(this._loop);
    window.clearInterval(this._videoloop);
  }

  private cpuStep() {
    if (!this._emulator) return;

    if (this._wasmContext)
      this._emulatorMemory = new Uint8Array(this._wasmContext.memory.buffer, this._emulator?.get_ram_ptr(), 0x4000);

    const prevMemAddress = this._emulator.get_last_ram_change();
    const prevMem = this.memory[prevMemAddress];

    this._emulator.execute_next();
    this._step += 1;

    const newMemAddress = this._emulator.get_last_ram_change();
    const ramChanged = prevMemAddress != newMemAddress || this.memory[prevMemAddress] != prevMem;

    if (this._cpmMode && this.emulator?.pc == 0x05) {
      // Emulate CP/M BDOS syscalls (https://www.seasip.info/Cpm/bdos.html)
      const registers = this.registers;
      const largeRegisters = this.largeRegisters;

      if (registers && largeRegisters) {
        const syscall = registers.c;

        switch(syscall) {
          case 2: {
            // BDOS function 2 (C_WRITE) - Console output
            const char = registers?.e;
            if (char) {
              console.log(String.fromCharCode(char));
            }
            break;
          }
          case 9: {
            // BDOS function 9 (C_WRITESTR) - Output string
            let address = largeRegisters.d;

            let currentChar = "";
            let fullString = "";
            while (currentChar != "$") {
              fullString += currentChar;
              currentChar = String.fromCharCode(this._emulatorMemory[address++]);
            }
            console.log(fullString);
            break;
          }
        }
      }
    }

    if (this._cpmMode && this.emulator?.pc == 0x00) {
      console.log("Entered CP/M Warm Boot, Stopping Emulator.");
      this.stop();
      return;
    }

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

  private isHalfVblank: boolean = false;

  private videoStep() {
    if (!this._emulator?.interrupts_enabled) return;
    if (this.isHalfVblank) {
      this._emulator?.interrupt(0xCF);
    } else {
      this._emulator?.interrupt(0xD7);
    }

    this.onVideoStep.emit();

    this.isHalfVblank = !this.isHalfVblank;
  }
}
