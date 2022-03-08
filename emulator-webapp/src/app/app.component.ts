import { Component, OnInit } from '@angular/core';
import init from "emulator";
import { MatIconRegistry } from '@angular/material/icon';
import { DomSanitizer } from '@angular/platform-browser';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.less']
})
export class AppComponent implements OnInit {

  title = 'emulator-webapp';

  constructor(private readonly matIconRegistry: MatIconRegistry, private readonly domSanitizer: DomSanitizer) {
    matIconRegistry.addSvgIcon("GitHub", domSanitizer.bypassSecurityTrustResourceUrl("assets/icons/github.svg"));
  }

  async ngOnInit(): Promise<void> {
    let emulator = await init("assets/emulator/emulator_bg.wasm");
  }

}
