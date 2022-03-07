import { Component, OnInit } from '@angular/core';
import init from "emulator";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.less']
})
export class AppComponent implements OnInit {

  title = 'emulator-webapp';
  editorOptions = {theme: 'vs-dark', language: 'javascript'};
  code: string = 'function x() {\nconsole.log("Hello world!");\n}';

  async ngOnInit(): Promise<void> {
    console.log("AppComponent OnInit!");
    let emulator = await init("assets/emulator/emulator_bg.wasm");
    //a.greet();
  }
}
