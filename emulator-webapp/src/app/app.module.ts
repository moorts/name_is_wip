import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { MonacoEditorModule } from '@materia-ui/ngx-monaco-editor';

import {MatToolbarModule} from '@angular/material/toolbar';
import {MatGridListModule} from '@angular/material/grid-list';
import {MatTabsModule} from '@angular/material/tabs';
import {MatIconModule} from '@angular/material/icon';
import {MatButtonModule} from '@angular/material/button';
import {MatInputModule} from '@angular/material/input';
import {MatDialogModule} from '@angular/material/dialog';
import {MatSelectModule} from '@angular/material/select';
import {MatRadioModule} from '@angular/material/radio';

import { AppComponent } from './app.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { AngularSplitModule } from 'angular-split';
import { FormsModule } from '@angular/forms';
import { HttpClientModule } from "@angular/common/http";

import { CodeEditorComponent } from './code-editor/code-editor.component';
import { RamDisplayComponent } from './ram-display/ram-display.component';
import { LoadFileDialogComponent } from './load-file-dialog/load-file-dialog.component';
import { VideoOutputComponent } from './video-output/video-output.component';

@NgModule({
  declarations: [
    AppComponent,
    CodeEditorComponent,
    RamDisplayComponent,
    LoadFileDialogComponent,
    VideoOutputComponent
  ],
  imports: [
    BrowserModule,
    MonacoEditorModule,
    BrowserAnimationsModule,
    MatToolbarModule,
    MatGridListModule,
    MatTabsModule,
    MatIconModule,
    MatButtonModule,
    MatInputModule,
    MatDialogModule,
    MatSelectModule,
    MatRadioModule,
    AngularSplitModule,
    FormsModule,
    HttpClientModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule {}
