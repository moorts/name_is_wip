import { Component, AfterViewInit, ViewChild, ElementRef } from '@angular/core';
import { EmulatorService } from '../emulator-service/emulator.service';

@Component({
  selector: 'ram-display',
  templateUrl: './ram-display.component.html',
  styleUrls: ['./ram-display.component.scss']
})
export class RamDisplayComponent implements AfterViewInit {

  public height = 16;
  public width = 16;

  private _offset: number = 0;
  private _rows: HTMLElement[] = [];
  private _cells: HTMLElement[] = [];
  private _previousHighlightedCell: HTMLElement | undefined;

  @ViewChild('registerB') registerB: ElementRef<HTMLInputElement> | undefined;
  @ViewChild('registerC') registerC: ElementRef<HTMLInputElement> | undefined;
  @ViewChild('registerD') registerD: ElementRef<HTMLInputElement> | undefined;
  @ViewChild('registerE') registerE: ElementRef<HTMLInputElement> | undefined;
  @ViewChild('registerH') registerH: ElementRef<HTMLInputElement> | undefined;
  @ViewChild('registerL') registerL: ElementRef<HTMLInputElement> | undefined;
  @ViewChild('registerA') registerA: ElementRef<HTMLInputElement> | undefined;

  @ViewChild('registerLargeB') registerLargeB: ElementRef<HTMLInputElement> | undefined;
  @ViewChild('registerLargeD') registerLargeD: ElementRef<HTMLInputElement> | undefined;
  @ViewChild('registerLargeH') registerLargeH: ElementRef<HTMLInputElement> | undefined;
  @ViewChild('registerLargePSW') registerLargePSW: ElementRef<HTMLInputElement> | undefined;
  @ViewChild('registerLargeSP') registerLargeSP: ElementRef<HTMLInputElement> | undefined;

  constructor(public readonly emulatorService: EmulatorService) {

  }

  ngAfterViewInit(): void {
    for(let y = 0; y < this.height; y++) {
      const row = document.getElementById("row_" + y);
      if (row) {
        this._rows[y] = row;
      }

      for(let x = 0; x < this.width; x++) {
        const cell = document.getElementById("cell_" + x + "_" + y);
        if (cell) {
          this._cells[y * this.width + x] = cell;
        }
      }
    }

    this.offset = 0;
    this.update(true);
  }

  public set offset(offset: number) {
    if (offset < 0) return;
    this._offset = offset;
    this._rows.forEach((row, index) => {
      row.innerText = (this._offset * 16 + index).toString(16).toUpperCase().padStart(3, '0') + "x";
    });
    this.update(true);
  }

  public get offset() {
    return this._offset;
  }

  public update(fullUpdate: boolean) {
    const registers = this.emulatorService.registers;
    if (registers) {
      if (this.registerB) this.registerB.nativeElement.value = registers.b.toString(16).toUpperCase().padStart(2, '0');
      if (this.registerC) this.registerC.nativeElement.value = registers.c.toString(16).toUpperCase().padStart(2, '0');
      if (this.registerD) this.registerD.nativeElement.value = registers.d.toString(16).toUpperCase().padStart(2, '0');
      if (this.registerE) this.registerE.nativeElement.value = registers.e.toString(16).toUpperCase().padStart(2, '0');
      if (this.registerH) this.registerH.nativeElement.value = registers.h.toString(16).toUpperCase().padStart(2, '0');
      if (this.registerL) this.registerL.nativeElement.value = registers.l.toString(16).toUpperCase().padStart(2, '0');
      if (this.registerA) this.registerA.nativeElement.value = registers.a.toString(16).toUpperCase().padStart(2, '0');
    } else {
      if (this.registerB) this.registerB.nativeElement.value = "00";
      if (this.registerC) this.registerC.nativeElement.value = "00";
      if (this.registerD) this.registerD.nativeElement.value = "00";
      if (this.registerE) this.registerE.nativeElement.value = "00";
      if (this.registerH) this.registerH.nativeElement.value = "00";
      if (this.registerL) this.registerL.nativeElement.value = "00";
      if (this.registerA) this.registerA.nativeElement.value = "00";
    }

    const largeRegisters = this.emulatorService.largeRegisters;
    if (largeRegisters) {
      if (this.registerLargeB) this.registerLargeB.nativeElement.value = largeRegisters.b.toString(16).toUpperCase().padStart(4, '0');
      if (this.registerLargeD) this.registerLargeD.nativeElement.value = largeRegisters.d.toString(16).toUpperCase().padStart(4, '0');
      if (this.registerLargeH) this.registerLargeH.nativeElement.value = largeRegisters.h.toString(16).toUpperCase().padStart(4, '0');
      if (this.registerLargePSW) this.registerLargePSW.nativeElement.value = largeRegisters.psw.toString(16).toUpperCase().padStart(4, '0');
      if (this.registerLargeSP) this.registerLargeSP.nativeElement.value = largeRegisters.sp.toString(16).toUpperCase().padStart(4, '0');
    } else {
      if (this.registerLargeB) this.registerLargeB.nativeElement.value = "0000";
      if (this.registerLargeD) this.registerLargeD.nativeElement.value = "0000";
      if (this.registerLargeH) this.registerLargeH.nativeElement.value = "0000";
      if (this.registerLargePSW) this.registerLargePSW.nativeElement.value = "0000";
      if (this.registerLargeSP) this.registerLargeSP.nativeElement.value = "0000";
    }

    const data = this.emulatorService.memory;

    if (fullUpdate) {
      // Update all cells
      for(let y = 0; y < this.height; y++) {
        for(let x = 0; x < this.width; x++) {
          const index = y * this.width + x;
          const dataIndex = this._offset * 16 + index;
          if (data.length > dataIndex) {
            this._cells[index].innerText = data[dataIndex].toString(16).toUpperCase().padStart(2, '0');
          } else {
            this._cells[index].innerText = "00";
          }
          this._cells[index].style.backgroundColor = "";
          this._cells[index].style.transition = "";
        }
      }
    } else {
      // Update only last changed cell

      if (this._previousHighlightedCell) {
        const prevCell = this._previousHighlightedCell;
        window.setTimeout(() => {
          prevCell.style.backgroundColor = "";
          prevCell.style.transition = "background-color 0.2s linear";
          window.setTimeout(() => {
            prevCell.style.transition = "";
          }, 200);
        }, 10);
      }

      const changedIndex = this.emulatorService.emulator?.get_last_ram_change() ?? 0;
      if (changedIndex < this._offset * 16 || changedIndex >= this._offset * 16 + this.width * this.height) return;
      const changedCell = this._cells[changedIndex - this._offset * 16];

      changedCell.innerText = data[changedIndex].toString(16).toUpperCase().padStart(2, '0');
      changedCell.style.backgroundColor = "#FF0000";

      this._previousHighlightedCell = changedCell;
    }
  }

  range(i: number) {
    return new Array(i);
  }

}
