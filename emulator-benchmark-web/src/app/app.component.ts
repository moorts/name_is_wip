import { Component, ElementRef, ViewChild } from '@angular/core';
import init, { createEmulator, InitOutput } from 'emulator';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  title = 'emulator-benchmark';

  constructor() {
    this.initialize();
  }

  private async initialize() {
    this.wasmContext = await init("assets/emulator/emulator_bg.wasm");
  }

  public running: boolean = false;
  public fileSelected: boolean = false;
  public finished: boolean = false;
  public totalTime: number = 0.0;
  public totalCycles: number = 0.0;
  public totalMHz: number = 0.0;

  @ViewChild("log") log: ElementRef<HTMLTextAreaElement> | undefined;

  private rom: ArrayBuffer | null = null;
  private wasmContext: InitOutput | null = null;

  public startBenchmark() {
    if (this.rom == null || this.wasmContext == null || this.log == undefined)
      return;

    const logOutput = this.log.nativeElement;

    this.finished = false;
    this.running = true;

    const largerBuffer = new ArrayBuffer(this.rom.byteLength + 0x100);
    const memory = new Uint8Array(largerBuffer);
    memory.set(new Uint8Array(this.rom), 0x100);
    memory[0x00] = 0xC9;
    memory[0x01] = 0x00;
    memory[0x05] = 0xC9;
    memory[0x06] = 0x01;
    memory[0x07] = 0xC9;

    const emulator = createEmulator(memory);

    const emulatorMemory = new Uint8Array(this.wasmContext.memory.buffer, emulator.get_ram_ptr());

    emulator.pc = 0x100;
    emulator.sp = 0xFF00;

    let totalCycles = 0;

    // Measure time to run
    const startTime = performance.now();

    while (emulator.pc != 0x00) {
      const cycles = emulator.execute_next();
      totalCycles += cycles;

      if (emulator.pc == 0x05) {
        const ptr = (<any>emulator.reg).ptr;
        const memory = new Uint8Array(this.wasmContext.memory.buffer, ptr);
        const largeMemory = new Uint16Array(this.wasmContext.memory.buffer, ptr);
        const c = memory[6];
        const e = memory[8];
        const de = largeMemory[4];

        if (c == 2) {
          //console.log(String.fromCharCode(e));
          logOutput.value += String.fromCharCode(e);
        } else if (c == 9) {
          let currentChar = "";
          let fullString = "";
          let address = de;
          while (currentChar != "$") {
            currentChar = String.fromCharCode(emulatorMemory[address++]);
            if (currentChar != "$")
              fullString += currentChar;
          }
          //console.log(fullString);
          logOutput.value += fullString;
        }
      }
    }

    const endTime = performance.now();

    this.totalTime = endTime - startTime;
    this.finished = true;
    this.totalCycles = totalCycles;
    this.totalMHz = totalCycles / this.totalTime / 1000;
  }

  public onFileSelected(event: Event) {
    const fileTarget = (<HTMLInputElement>event.target);
    if (!fileTarget.files || !fileTarget.files[0])
      return;

    const reader = new FileReader();
    reader.onload = () => {
      this.fileSelected = true;
      this.rom = <ArrayBuffer>reader.result;
    }
    reader.readAsArrayBuffer(fileTarget.files[0]);
    console.log("Loading file...");
  }
}
