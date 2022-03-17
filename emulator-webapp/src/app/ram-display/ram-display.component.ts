import { Component, AfterViewInit } from '@angular/core';
import { EmulatorService } from '../emulator-service/emulator.service';

@Component({
  selector: 'ram-display',
  templateUrl: './ram-display.component.html',
  styleUrls: ['./ram-display.component.less']
})
export class RamDisplayComponent implements AfterViewInit {

  public offset: number = 0;
  public height = 16;
  public width = 16;

  private rows: HTMLElement[] = [];
  private cells: HTMLElement[] = [];

  constructor(private readonly emulatorService: EmulatorService) {

  }

  ngAfterViewInit(): void {
    for(let y = 0; y < this.height; y++) {
      const row = document.getElementById("row_" + y);
      if (row) {
        this.rows[y] = row;
      }

      for(let x = 0; x < this.width; x++) {
        const cell = document.getElementById("cell_" + x + "_" + y);
        if (cell) {
          this.cells[y * this.width + x] = cell;
        }
      }
    }

    this.update();
  }

  public update() {
    this.rows.forEach((row, index) => {
      row.innerText = (this.offset + index).toString(16).toUpperCase().padStart(3, '0') + "x";
    });

    const data = this.emulatorService.memory;

    for(let y = 0; y < this.height; y++) {
      for(let x = 0; x < this.width; x++) {
        const index = y * this.width + x;
        const dataIndex = this.offset + index;
        if (data.length > dataIndex) {
          this.cells[index].innerText = data[dataIndex].toString(16).toUpperCase().padStart(2, '0');
        } else {
          this.cells[index].innerText = "00";
        }
      }
    }
  }

  range(i: number) {
    return new Array(i);
  }

}
