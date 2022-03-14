import { Component, ViewChild } from '@angular/core';
import { MatIconRegistry } from '@angular/material/icon';
import { DomSanitizer } from '@angular/platform-browser';
import { EmulatorService } from './emulator-service/emulator.service';
import { CodeEditorComponent } from './code-editor/code-editor.component';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.less']
})
export class AppComponent {

  title = 'emulator-webapp';

  @ViewChild('codeEditor') codeEditor: CodeEditorComponent | undefined;

  constructor(private readonly matIconRegistry: MatIconRegistry,
              private readonly domSanitizer: DomSanitizer,
              private readonly emulatorService: EmulatorService) {
    matIconRegistry.addSvgIcon("GitHub", domSanitizer.bypassSecurityTrustResourceUrl("assets/icons/github.svg"));
  }

  public onAssembleButtonPressed() {
    this.emulatorService.assemble(this.codeEditor?.code ?? "");
  }
}
