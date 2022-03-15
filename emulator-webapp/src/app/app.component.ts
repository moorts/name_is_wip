import { AfterViewInit, Component, ElementRef, Renderer2, ViewChild } from '@angular/core';
import { MatIconRegistry } from '@angular/material/icon';
import { DomSanitizer } from '@angular/platform-browser';
import { EmulatorService } from './emulator-service/emulator.service';
import { CodeEditorComponent } from './code-editor/code-editor.component';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.less']
})
export class AppComponent implements AfterViewInit {

  title = 'emulator-webapp';

  @ViewChild('codeEditor') codeEditor: CodeEditorComponent | undefined;
  @ViewChild('fileDialog') fileDialog: ElementRef | undefined;

  constructor(private readonly matIconRegistry: MatIconRegistry,
              private readonly domSanitizer: DomSanitizer,
              private readonly emulatorService: EmulatorService,
              private readonly renderer: Renderer2) {
    matIconRegistry.addSvgIcon("GitHub", domSanitizer.bypassSecurityTrustResourceUrl("assets/icons/github.svg"));
  }

  ngAfterViewInit(): void {
    if (this.fileDialog != null) {
      this.renderer.listen(this.fileDialog.nativeElement, "change", e => this.handleFileSelect(this, e));
    }
  }

  public onAssembleButtonPressed() {
    this.emulatorService.assemble(this.codeEditor?.code ?? "");
  }

  public onFileOpenButtonPressed() {
    this.fileDialog?.nativeElement.click();
  }

  private handleFileSelect (app: AppComponent, e: any) {
    var files = e.target.files;
    if (files.length < 1) {
        return;
    }
    var file = files[0];
    var reader = new FileReader();
    reader.onload = e => app.onFileLoaded(app, e);
    reader.readAsDataURL(file);
  }

  private onFileLoaded (app: AppComponent, e: ProgressEvent<FileReader>) {
    var match = /^data:(.*);base64,(.*)$/.exec(e.target?.result?.toString() ?? "");
    if (match == null) {
        throw 'Could not parse result'; // should not happen
    }
    var content = match[2];
    if (app.codeEditor != null) {
      app.codeEditor.code = atob(content);
    }
  }
}
