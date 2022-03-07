import { Component, OnInit } from '@angular/core';
import { MonacoEditorLoaderService, MonacoStandaloneCodeEditor } from '@materia-ui/ngx-monaco-editor';
import init from "emulator";
import { loadWASM } from 'onigasm';
import { filter, take } from 'rxjs';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.less']
})
export class AppComponent implements OnInit {

  title = 'emulator-webapp';
  editorOptions = {theme: 'vs-dark', language: ''};
  code: string = 'function x() {\nconsole.log("Hello world!");\n}';

  constructor(private monacoLoaderService: MonacoEditorLoaderService) {
    this.monacoLoaderService.isMonacoLoaded$.pipe(
      filter(isLoaded => isLoaded),
      take(1),
    ).subscribe(() => {
         
    });
   }

  async ngOnInit(): Promise<void> {
    console.log("AppComponent OnInit!");
    await loadWASM('assets/onigasm/lib/onigasm.wasm')
    let emulator = await init("assets/emulator/emulator_bg.wasm");
    //a.greet();
  }

  editorInit(editor: MonacoStandaloneCodeEditor) {
    editor.addAction({
        id: "EMULATOR_ASSEMBLE", 
        label: "Emulator: Assemble", 
        run: () => {
          console.log("nice cock!");
        }
      });
  }
}
