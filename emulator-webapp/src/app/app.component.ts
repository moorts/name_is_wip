import { AfterViewInit, Component, ElementRef, Renderer2, ViewChild } from '@angular/core';
import { MatIconRegistry } from '@angular/material/icon';
import { DomSanitizer } from '@angular/platform-browser';
import { EmulatorService } from './emulator-service/emulator.service';
import { CodeEditorComponent } from './code-editor/code-editor.component';
import { RamDisplayComponent } from './ram-display/ram-display.component';
import { ThemeService } from './theme-service/theme.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements AfterViewInit {

  title = 'emulator-webapp';

  @ViewChild('codeEditor') codeEditor: CodeEditorComponent | undefined;
  @ViewChild('ramDisplay') ramDisplay: RamDisplayComponent | undefined;
  @ViewChild('fileDialog') fileDialog: ElementRef | undefined;

  constructor(private readonly matIconRegistry: MatIconRegistry,
              private readonly domSanitizer: DomSanitizer,
              public readonly emulatorService: EmulatorService,
              private readonly renderer: Renderer2,
              private readonly themeService: ThemeService) {
    matIconRegistry.addSvgIcon("GitHub", domSanitizer.bypassSecurityTrustResourceUrl("assets/icons/github.svg"));
    emulatorService.onStep.subscribe((props) => {
      this.ramDisplay?.update(false, props.ramChanged);
    });
  }

  ngAfterViewInit(): void {
    if (this.fileDialog != null) {
      this.renderer.listen(this.fileDialog.nativeElement, "change", e => this.handleFileSelect(this, e));
    }
  }

  public onAssembleButtonPressed() {
    this.emulatorService.assemble(this.codeEditor?.code ?? "");
    this.ramDisplay?.update(true);
  }

  public onFileOpenButtonPressed() {
    this.fileDialog?.nativeElement.click();
  }

  public onPlayButtonPressed() {
    this.emulatorService.start();
  }

  public onStopButtonPressed() {
    this.emulatorService.stop();
    this.ramDisplay?.update(true);
  }

  public onPauseButtonPressed() {
    this.emulatorService.togglePause();
  }

  public onStepButtonPressed() {
    this.emulatorService.doStep();
  }

  public onThemeButtonPressed() {
    this.themeService.toggleTheme();
  }

  private handleFileSelect (app: AppComponent, e: any) {
    var files = e.target.files;
    if (files.length < 1) {
        return;
    }
    var file = files[0];
    var reader = new FileReader();
    reader.onload = e => app.onFileLoaded(app, e, file);
    reader.readAsArrayBuffer(file);
  }

  private onFileLoaded (app: AppComponent, e: ProgressEvent<FileReader>, file: any) {
    const buffer = e.target?.result;
    if (!(buffer instanceof ArrayBuffer)) return;

    const isROM = (<string>file.name).toLowerCase().endsWith(".com");

    if (!isROM && app.codeEditor != null) {
      app.codeEditor.code = new TextDecoder().decode(buffer);
    }
    if (isROM && app.codeEditor != null) {
      const largerBuffer = new ArrayBuffer(buffer.byteLength + 0x100);
      new Uint8Array(largerBuffer).set(new Uint8Array(buffer), 0x100);
      app.emulatorService.loadBytes(largerBuffer);
      const newCode = app.emulatorService.disassemble(new Uint8Array(largerBuffer));
      app.codeEditor.code = newCode;
      app.ramDisplay?.update(true);
    }
  }
}
