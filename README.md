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


## Quickstart

This section will set you up with the application as quick as possible to run it on your machine.

### Prerequisites

These are the things you will need to install beforehand (and most likely already have if you are interested in running this project):

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://github.com/rustwasm/wasm-pack)
- [Node.js](https://nodejs.org/en/download/)

To get all dependencies used in the frontend you will need to have Yarn installed, which is done easiest with npm:
```
npm install --global yarn
```

### Steps

0. You can run the unittests included in the backends [src](./emulator/src/)-directory with ```cargo test```

1. Navigate to the [emulator](./emulator/) and build the wasm dependencies:
```
wasm-pack build -t web
```

2. Head over to the [frontend-directory](./emulator-webapp/) and install all required dependencies:
```
yarn install
```

3. From there, deploy the project with:
```
npm run start
```
This will start the emulator as an Angular application on your machine. If left unchanged it will be available on 'localhost:4200'.
