import { Component, OnInit } from '@angular/core';
import { MonacoEditorLoaderService, MonacoStandaloneCodeEditor } from '@materia-ui/ngx-monaco-editor';
import { filter, take } from 'rxjs';

@Component({
  selector: 'code-editor',
  templateUrl: './code-editor.component.html',
  styleUrls: ['./code-editor.component.less']
})
export class CodeEditorComponent implements OnInit {

  editorOptions = {theme: 'vs-dark', language: ''};
  code: string = 'MOV A, B';

  constructor(private monacoLoaderService: MonacoEditorLoaderService) {
    this.monacoLoaderService.isMonacoLoaded$.pipe(
      filter(isLoaded => isLoaded),
      take(1),
    ).subscribe(() => {

    });
   }

  async ngOnInit(): Promise<void> {

  }

  async editorInit(editor: MonacoStandaloneCodeEditor) {
    editor.addAction({
      id: "EMULATOR_ASSEMBLE",
      label: "Emulator: Assemble",
      run: () => {
        console.log("nice cock!");
      }
    });

    monaco.languages.register({ id: "i8080" });

    monaco.languages.setMonarchTokensProvider('i8080', {
      keywords: [
        "STC", "CMC", "INR", "DCR", "CMA", "DAA", "NOP", "MOV", "STAX", "LDAX", "ADD", "ADC",
        "SUB", "SBB", "ANA", "XRA", "ORA", "CMP", "RLC", "RRC", "RAL", "RAR", "PUSH", "POP",
        "DAD", "INX", "DCX", "XCHG", "XTHL", "SPHL", "LXI", "MVI", "ADI", "ACI", "SUI", "SBI",
        "ANI", "XRI", "ORI", "CPI", "STA", "LDA", "SHLD", "LHLD", "PCHL", "JMP", "JC", "JNC",
        "JZ", "JNZ", "JP", "JM", "JPE", "JPO", "CALL", "CC", "CNC", "CZ", "CNZ", "CP", "CM",
        "CPE", "CPO", "RET", "RC", "RNC", "RZ", "RNZ", "RM", "RP", "RPE", "RPO", "RST", "EI",
        "DI", "IN", "OUT", "HLT"
      ],
      registers: [
        "B", "C", "D", "H", "L", "A", "SP", "PSW"
      ],
      preprocessor: [
        "ORG", "EQU", "SET", "END", "IF", "ENDIF", "MACRO", "ENDM"
      ],
      tokenizer: {
        root: [
          [/;.*/, 'comment'],
          [/[a-z_$A-Z][\w$]*/, { cases: { '@keywords': 'keyword',
                                       '@preprocessor': 'preprocessor',
                                       '@registers': 'register',
                                       '@default': 'identifier' } }],
        ]
      }
    });

    monaco.editor.defineTheme('i8080theme', {
      base: 'vs-dark',
      inherit: true,
      rules: [
        { token: 'comment', foreground: '#6A9955' },
        { token: 'register', foreground: '#ce9178' },
        { token: 'keyword', foreground: '#569cd6' },
        { token: 'preprocessor', foreground: '#d16969' },
      ],
      colors: {
        'editor.foreground': '#FFFFFF'
      }
    });

    let model = editor.getModel()
    if (model != null) {
      monaco.editor.setModelLanguage(model, "i8080");
      monaco.editor.setTheme("i8080theme");
    }

  }

}
