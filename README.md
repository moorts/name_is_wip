# Some emulator

## Resources

Opcode table:
[Opcode Cheatsheet](https://tobiasvl.github.io/optable/intel-8080/classic)

Data sheet:
[Data sheet](https://altairclone.com/downloads/manuals/8080%20Data%20Sheet.pdf)

## Idee

Entwicklung eines Intel 8080 Emulators in Rust, kompiliert nach Web-Assembly. Dazu ein Web-Frontend durch das der Emulator verwendet werden kann. Das UI soll leicht verstaendlich sein, sodass die Anwendung als Grundlage fuer Assembly-Kurse verwendet werden kann. Der Emulator laeuft als WASM binary im Browser des Benutzers und wird durch die WASM-Endpunkte gesteuert.

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
* Fazit
