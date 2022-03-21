import { EventEmitter, Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class ThemeService {

  public onThemeSwitched: EventEmitter<any> = new EventEmitter();

  private _currentTheme: "dark" | "light" = "dark";

  public get currentTheme() {
    return this._currentTheme;
  }

  constructor() {
  }

  public toggleTheme() {
    if (this._currentTheme == "dark") {
      this._currentTheme = "light";
    } else {
      this._currentTheme = "dark";
    }

    this.onThemeSwitched.emit();
    this.updateTheme();
  }

  private updateTheme() {
    const matSheet = document.getElementById("themeMaterialCss");
    const appSheet = document.getElementById("themeAppCss");
    if (!matSheet || !appSheet) return;

    if (this._currentTheme == "dark") {
      matSheet.setAttribute("href", "assets/angular/themes/purple-green.css");
      appSheet.setAttribute("href", "assets/styles-dark.css");
    } else {
      matSheet.setAttribute("href", "assets/angular/themes/deeppurple-amber.css");
      appSheet.setAttribute("href", "assets/styles-light.css");
    }
  }
}
