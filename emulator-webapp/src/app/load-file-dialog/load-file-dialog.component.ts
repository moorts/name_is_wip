import { Component, OnInit } from '@angular/core';
import { MatDialogRef } from '@angular/material/dialog';
import { AppComponent } from '../app.component';

@Component({
  selector: 'app-load-file-dialog',
  templateUrl: './load-file-dialog.component.html',
  styleUrls: ['./load-file-dialog.component.less']
})
export class LoadFileDialogComponent implements OnInit {

  public fileType: "asm"|"rom" = "asm";
  public startPosition: number = 0;

  public app: AppComponent | undefined;

  constructor(private readonly dialogRef: MatDialogRef<LoadFileDialogComponent>) { }

  ngOnInit(): void {

  }

  public log() {
    console.log(this.fileType);
  }

  onFileSelected() {
    const inputNode: any = document.querySelector('#file');

    if (typeof (FileReader) !== 'undefined') {
      const reader = new FileReader();

      reader.onload = (e: any) => {
        const buffer = e.target?.result;
        if (!(buffer instanceof ArrayBuffer)) return;

        if (this.fileType == 'asm' && this.app?.codeEditor) {
          this.app.codeEditor.code = new TextDecoder().decode(buffer);
        }
        if (this.fileType == 'rom' && this.app?.codeEditor) {
          const largerBuffer = new ArrayBuffer(buffer.byteLength + this.startPosition);
          new Uint8Array(largerBuffer).set(new Uint8Array(buffer), this.startPosition);
          this.app.emulatorService.loadBytes(largerBuffer);
          const newCode = this.app.emulatorService.disassemble(new Uint8Array(largerBuffer));
          this.app.codeEditor.code = newCode;
          this.app.ramDisplay?.update(true);
        }

        this.dialogRef.close();
      };

      reader.readAsArrayBuffer(inputNode.files[0]);
    }
  }
}
