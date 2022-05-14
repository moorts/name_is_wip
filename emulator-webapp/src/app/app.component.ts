import { AfterViewInit, Component, ElementRef, Renderer2, ViewChild } from '@angular/core';
import { MatIconRegistry } from '@angular/material/icon';
import { DomSanitizer } from '@angular/platform-browser';
import { EmulatorService } from './emulator-service/emulator.service';
import { CodeEditorComponent } from './code-editor/code-editor.component';
import { RamDisplayComponent } from './ram-display/ram-display.component';
import { ThemeService } from './theme-service/theme.service';
import { MatDialog } from '@angular/material/dialog';
import { LoadFileDialogComponent } from './load-file-dialog/load-file-dialog.component';
import { VideoOutputComponent } from './video-output/video-output.component';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements AfterViewInit {

  title = 'emulator-webapp';

  @ViewChild('codeEditor') public codeEditor: CodeEditorComponent | undefined;
  @ViewChild('ramDisplay') public ramDisplay: RamDisplayComponent | undefined;
  @ViewChild('videoOutput') public videoOutput: VideoOutputComponent | undefined;

  constructor(private readonly matIconRegistry: MatIconRegistry,
              private readonly domSanitizer: DomSanitizer,
              public readonly emulatorService: EmulatorService,
              private readonly renderer: Renderer2,
              private readonly themeService: ThemeService,
              private readonly dialog: MatDialog ) {
    matIconRegistry.addSvgIcon("GitHub", domSanitizer.bypassSecurityTrustResourceUrl("assets/icons/github.svg"));
    emulatorService.onStep.subscribe((props) => {
      this.ramDisplay?.update(false, props.ramChanged);
      this.codeEditor?.update();
    });
    emulatorService.onVideoStep.subscribe(() => {
      this.videoOutput?.update();
    });
  }

  ngAfterViewInit(): void {
    this.videoOutput?.update();
  }

  public onAssembleButtonPressed() {
    this.emulatorService.assemble(this.codeEditor?.code ?? "");
    this.ramDisplay?.update(true);
  }

  public onFileOpenButtonPressed() {
    const fileDialog = this.dialog.open(LoadFileDialogComponent, {
      height: '250px',
      width: '500px',
    });
    fileDialog.componentInstance.app = this;
  }

  public onPlayButtonPressed() {
    this.emulatorService.start();
    this.ramDisplay?.update(true);
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
}
