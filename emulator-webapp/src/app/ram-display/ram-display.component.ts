import { Component, AfterViewInit } from '@angular/core';
import { EmulatorService } from '../emulator-service/emulator.service';

@Component({
  selector: 'ram-display',
  templateUrl: './ram-display.component.html',
  styleUrls: ['./ram-display.component.less']
})
export class RamDisplayComponent implements AfterViewInit {

  public height = 16;
  public width = 16;

  private _offset: number = 0;
  private _rows: HTMLElement[] = [];
  private _cells: HTMLElement[] = [];
  private _previousHighlightedCell: HTMLElement | undefined;

  constructor(private readonly emulatorService: EmulatorService) {

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
    this._offset = offset;
    this._rows.forEach((row, index) => {
      row.innerText = (this._offset + index).toString(16).toUpperCase().padStart(3, '0') + "x";
    });
  }

  public update(fullUpdate: boolean) {
    const data = this.emulatorService.memory;

    if (fullUpdate) {
      // Update all cells
      for(let y = 0; y < this.height; y++) {
        for(let x = 0; x < this.width; x++) {
          const index = y * this.width + x;
          const dataIndex = this._offset + index;
          if (data.length > dataIndex) {
            this._cells[index].innerText = data[dataIndex].toString(16).toUpperCase().padStart(2, '0');
          } else {
            this._cells[index].innerText = "00";
          }
        }
      }
    } else {
      // Update only last changed cell
      const changedIndex = this.emulatorService.emulator?.get_last_ram_change() ?? 0;
      if (changedIndex < this._offset || changedIndex > this._offset + this.width * this.height) return;
      const changedCell = this._cells[changedIndex - this._offset];

      changedCell.innerText = data[changedIndex].toString(16).toUpperCase().padStart(2, '0');
      changedCell.style.backgroundColor = "#FF0000";

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

      this._previousHighlightedCell = changedCell;
    }
  }

  range(i: number) {
    return new Array(i);
  }

}
