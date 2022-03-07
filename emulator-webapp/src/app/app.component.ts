import { Component, OnInit } from '@angular/core';
import init from "emulator";

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
