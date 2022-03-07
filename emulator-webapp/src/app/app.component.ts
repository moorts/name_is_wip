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

  async ngOnInit(): Promise<void> {
    let emulator = await init("assets/emulator/emulator_bg.wasm");
  }

}
