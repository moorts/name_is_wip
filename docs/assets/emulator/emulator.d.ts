/* tslint:disable */
/* eslint-disable */
/**
* @param {string} code
* @returns {Uint8Array}
*/
export function assemble(code: string): Uint8Array;
/**
* @param {string} code
* @returns {any}
*/
export function get_linemap(code: string): any;
/**
* @param {Uint8Array} bytes
* @returns {string}
*/
export function disassemble(bytes: Uint8Array): string;
/**
* @param {Uint8Array} memory
* @returns {Emulator}
*/
export function createEmulator(memory: Uint8Array): Emulator;
/**
*/
export class Emulator {
  free(): void;
/**
* @returns {Emulator}
*/
  static new(): Emulator;
/**
* @returns {number}
*/
  get_ram_ptr(): number;
/**
* @returns {number}
*/
  get_last_ram_change(): number;
/**
* @returns {number}
*/
  execute_next(): number;
/**
* @param {Uint8Array} data
* @param {number} start
*/
  load_ram(data: Uint8Array, start: number): void;
/**
* @param {number} opcode
* @returns {number}
*/
  interrupt(opcode: number): number;
/**
*/
  interrupts_enabled: boolean;
/**
*/
  pc: number;
/**
*/
  reg: RegisterArray;
/**
*/
  running: boolean;
/**
*/
  sp: number;
}
/**
*/
export class RegisterArray {
  free(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_emulator_free: (a: number) => void;
  readonly __wbg_get_emulator_pc: (a: number) => number;
  readonly __wbg_set_emulator_pc: (a: number, b: number) => void;
  readonly __wbg_get_emulator_sp: (a: number) => number;
  readonly __wbg_set_emulator_sp: (a: number, b: number) => void;
  readonly __wbg_get_emulator_reg: (a: number) => number;
  readonly __wbg_set_emulator_reg: (a: number, b: number) => void;
  readonly __wbg_get_emulator_running: (a: number) => number;
  readonly __wbg_set_emulator_running: (a: number, b: number) => void;
  readonly __wbg_get_emulator_interrupts_enabled: (a: number) => number;
  readonly __wbg_set_emulator_interrupts_enabled: (a: number, b: number) => void;
  readonly emulator_new: () => number;
  readonly emulator_get_ram_ptr: (a: number) => number;
  readonly emulator_get_last_ram_change: (a: number) => number;
  readonly emulator_execute_next: (a: number, b: number) => void;
  readonly emulator_load_ram: (a: number, b: number, c: number, d: number) => void;
  readonly emulator_interrupt: (a: number, b: number, c: number) => void;
  readonly __wbg_registerarray_free: (a: number) => void;
  readonly assemble: (a: number, b: number, c: number) => void;
  readonly get_linemap: (a: number, b: number) => number;
  readonly disassemble: (a: number, b: number, c: number) => void;
  readonly createEmulator: (a: number, b: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
