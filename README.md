# Some emulator

![build](https://img.shields.io/github/workflow/status/moorts/name_is_wip/Rust/main)
![GitHub](https://img.shields.io/github/license/moorts/name_is_wip)

## Resources

Opcode table:
[Opcode Cheatsheet](https://tobiasvl.github.io/optable/intel-8080/classic)

Opcodes 0x00-0xff:
[Better Cheatsheet](http://www.emulator101.com/reference/8080-by-opcode.html)

Data sheet:
[Data sheet](https://altairclone.com/downloads/manuals/8080%20Data%20Sheet.pdf)

8080 Programming:
[Programming Manual](https://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf)

Rust Web-Assembly:
[Official Tutorial](https://rustwasm.github.io/docs/book/)

## Idee

Entwicklung eines Intel 8080 Emulators in Rust, kompiliert nach Web-Assembly. Dazu ein Web-Frontend durch das der Emulator verwendet werden kann. Das UI soll leicht verständlich sein, sodass die Anwendung als Grundlage fuer Assembly-Kurse verwendet werden kann. Der Emulator läuft als WASM binary im Browser des Benutzers und wird durch die WASM-Endpunkte gesteuert.

## ToC

* Analysis
  * Problemstellung
  * Verwandte Arbeiten
  * Ziele
  * Beitrag
* Design
  * Emulator
    * Disassembler
    * Assembler
    * Interpreter
    * CPU Emulation
  * WASM API
  * Frontend
    * Framework
    * SPA vs non-SPA
    * Architektur
      * Views
      * Components
* Implementierung
  * Rust
  * Interessante Code Beispiele
* Auswertung
  * Performance
  * Benutzbarkeit
  * Vollstaendigkeit
* Future Work
* Fazit

