import { Component, EventEmitter, OnInit, Output } from '@angular/core';
import { MonacoEditorLoaderService, MonacoStandaloneCodeEditor } from '@materia-ui/ngx-monaco-editor';
import { filter, take } from 'rxjs';
import { EmulatorService } from '../emulator-service/emulator.service';

@Component({
  selector: 'code-editor',
  templateUrl: './code-editor.component.html',
  styleUrls: ['./code-editor.component.less']
})
export class CodeEditorComponent implements OnInit {

  @Output() assembleAction: EventEmitter<void> = new EventEmitter<void>();
  @Output() startAction: EventEmitter<void> = new EventEmitter<void>();
  @Output() pauseAction: EventEmitter<void> = new EventEmitter<void>();
  @Output() stopAction: EventEmitter<void> = new EventEmitter<void>();

  public code: string = `LXI H, 0010H
MOV M, L
INX H
JMP 2
END`;

  editorOptions = {theme: 'vs-dark', language: ''};

  constructor(private readonly monacoLoaderService: MonacoEditorLoaderService,
              private readonly emulatorService: EmulatorService) {
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
        this.assembleAction.emit();
      }
    });

    editor.addAction({
      id: "EMULATOR_RUN",
      label: "Emulator: Start emulation",
      run: () => {
        this.startAction.emit();
      }
    });

    editor.addAction({
      id: "EMULATOR_PAUSE",
      label: "Emulator: Pause emulation",
      run: () => {
        this.pauseAction.emit();
      }
    });

    editor.addAction({
      id: "EMULATOR_STOP",
      label: "Emulator: Stop emulation",
      run: () => {
        this.stopAction.emit();
      }
    });

    monaco.languages.register({ id: "i8080" });

    const keywords = [
      "STC", "CMC", "INR", "DCR", "CMA", "DAA", "NOP", "MOV", "STAX", "LDAX", "ADD", "ADC",
      "SUB", "SBB", "ANA", "XRA", "ORA", "CMP", "RLC", "RRC", "RAL", "RAR", "PUSH", "POP",
      "DAD", "INX", "DCX", "XCHG", "XTHL", "SPHL", "LXI", "MVI", "ADI", "ACI", "SUI", "SBI",
      "ANI", "XRI", "ORI", "CPI", "STA", "LDA", "SHLD", "LHLD", "PCHL", "JMP", "JC", "JNC",
      "JZ", "JNZ", "JP", "JM", "JPE", "JPO", "CALL", "CC", "CNC", "CZ", "CNZ", "CP", "CM",
      "CPE", "CPO", "RET", "RC", "RNC", "RZ", "RNZ", "RM", "RP", "RPE", "RPO", "RST", "EI",
      "DI", "IN", "OUT", "HLT"
    ];

    const smallRegisters = [
      "B", "C", "D", "E", "H", "L", "A", "M"
    ];

    const largeRegisters = [
      "B", "D", "H", "PSW", "SP"
    ];

    const preprocessor = [
      "ORG", "EQU", "SET", "END", "IF", "ENDIF", "MACRO", "ENDM"
    ];

    monaco.languages.setMonarchTokensProvider('i8080', {
      keywords: keywords,
      registers: smallRegisters.concat(largeRegisters),
      preprocessor: preprocessor,
      ignoreCase: true,
      tokenizer: {
        root: [
          [/;.*/, 'comment'],
          [/^(\w+):/, 'label'],
          [/[a-z_$A-Z][\w$]*/, { cases: { '@keywords': 'keyword',
                                       '@preprocessor': 'preprocessor',
                                       '@registers': 'register',
                                       '@default': 'identifier' } }],
          [/[0-9a-fA-F]+[hHbBoOqQdD]?/, 'number'],
        ]
      }
    });

    monaco.editor.defineTheme('i8080theme', {
      base: 'vs-dark',
      inherit: true,
      rules: [
        { token: 'comment', foreground: '#6A9955' },
        { token: 'register', foreground: '#CE9178' },
        { token: 'keyword', foreground: '#569CD6' },
        { token: 'preprocessor', foreground: '#D16969' },
        { token: 'label', foreground: '#AAAAAA' },
        { token: 'number', foreground: '#93CEA8' },
      ],
      colors: {
        'editor.foreground': '#FFFFFF'
      }
    });

    const descriptions = new Map<string, string>();

    // Following lines shamelessly copied from https://github.com/mborik/i8080-macroasm-vscode/blob/master/src/defs_regex.ts
    // TODO: Are we going to jail?
    const mkRegex = (str: TemplateStringsArray, opts: string = 'i') => new RegExp(str.raw[0].replace(/\s/gm, ''), opts);
    const shouldSuggestInstruction = /^(((\$\$(?!\.))?[\w\.]+):)?\s*(\w+)?(?!.+)$/;
    const shouldSuggest1ArgRegister = mkRegex`
		(?:
			(pop|push|dad|ldax|lxi|stax|inx|dcx)|
			(ad[cd]|s[bu]b|ana|ora|xra|cmp|mov|mvi|inr|dcr)
		)
		\s+([a-z]\w*)?$`;
    const shouldSuggest2ArgRegister = mkRegex`
		(lxi|mvi|mov)
		\s+(\w+)(,\s*?[^\n$]*)$`;

    monaco.languages.registerCompletionItemProvider('i8080', {
      provideCompletionItems: function (model, position) {

        const line = model.getValueInRange({
          startLineNumber: position.lineNumber,
          startColumn: 1,
          endLineNumber: position.lineNumber,
          endColumn: position.column
        });
        const word = model.getWordUntilPosition(position);

        const instructionMatch = shouldSuggestInstruction.exec(line);
        if (instructionMatch) {
          const start = instructionMatch[4];
          let isUppercase = true;
          if (start) {
            const firstChar = start[0];
            isUppercase = firstChar == firstChar.toUpperCase();
          }
          return {
            suggestions: keywords.concat(preprocessor).map(i => {
              const instruction = isUppercase ? i.toUpperCase() : i.toLowerCase();
              return {
                label: instruction,
                kind: monaco.languages.CompletionItemKind.Method,
                documentation: descriptions.get(i) ?? "",
                insertText: instruction,
                range: {
                  startLineNumber: position.lineNumber,
                  endLineNumber: position.lineNumber,
                  startColumn: word.startColumn,
                  endColumn: word.endColumn
                }
              }
            })
          }
        }

        const shouldSuggest1ArgRegisterMatch = shouldSuggest1ArgRegister.exec(line);
		    const shouldSuggest2ArgRegisterMatch = shouldSuggest2ArgRegister.exec(line);

        if (shouldSuggest2ArgRegisterMatch) {
          const start = shouldSuggest2ArgRegisterMatch[1];
          let isUppercase = true;
          if (start) {
            const firstChar = start[0];
            isUppercase = firstChar == firstChar.toUpperCase();
          }
          return {
            suggestions: smallRegisters.map(i => {
              const register = isUppercase ? i.toUpperCase() : i.toLowerCase();
              return {
                label: register,
                kind: monaco.languages.CompletionItemKind.Property,
                documentation: descriptions.get(i) ?? "",
                insertText: register,
                range: {
                  startLineNumber: position.lineNumber,
                  endLineNumber: position.lineNumber,
                  startColumn: word.startColumn,
                  endColumn: word.endColumn
                }
              }
            })
          };
        } else if (shouldSuggest1ArgRegisterMatch) {
          const start = shouldSuggest1ArgRegisterMatch[0];
          let isUppercase = true;
          if (start) {
            const firstChar = start[0];
            isUppercase = firstChar == firstChar.toUpperCase();
          }

          let registers = smallRegisters;
          if (shouldSuggest1ArgRegisterMatch[1]) {
            registers = largeRegisters;
          }

          return {
            suggestions: registers.map(i => {
              const register = isUppercase ? i.toUpperCase() : i.toLowerCase();
              return {
                label: register,
                kind: monaco.languages.CompletionItemKind.Property,
                documentation: descriptions.get(i) ?? "",
                insertText: register,
                range: {
                  startLineNumber: position.lineNumber,
                  endLineNumber: position.lineNumber,
                  startColumn: word.startColumn,
                  endColumn: word.endColumn
                }
              }
            })
          };
        }

        return { suggestions: [] };
      }
    });

    let model = editor.getModel()
    if (model != null) {
      monaco.editor.setModelLanguage(model, "i8080");
      monaco.editor.setTheme("i8080theme");
    }

  }

}
