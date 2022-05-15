/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function __wbg_emulator_free(a: number): void;
export function __wbg_get_emulator_pc(a: number): number;
export function __wbg_set_emulator_pc(a: number, b: number): void;
export function __wbg_get_emulator_sp(a: number): number;
export function __wbg_set_emulator_sp(a: number, b: number): void;
export function __wbg_get_emulator_reg(a: number): number;
export function __wbg_set_emulator_reg(a: number, b: number): void;
export function __wbg_get_emulator_running(a: number): number;
export function __wbg_set_emulator_running(a: number, b: number): void;
export function __wbg_get_emulator_interrupts_enabled(a: number): number;
export function __wbg_set_emulator_interrupts_enabled(a: number, b: number): void;
export function emulator_new(): number;
export function emulator_get_ram_ptr(a: number): number;
export function emulator_get_last_ram_change(a: number): number;
export function emulator_execute_next(a: number, b: number): void;
export function emulator_load_ram(a: number, b: number, c: number, d: number): void;
export function emulator_interrupt(a: number, b: number, c: number): void;
export function __wbg_registerarray_free(a: number): void;
export function assemble(a: number, b: number, c: number): void;
export function get_linemap(a: number, b: number): number;
export function disassemble(a: number, b: number, c: number): void;
export function createEmulator(a: number, b: number): number;
export function __wbindgen_add_to_stack_pointer(a: number): number;
export function __wbindgen_malloc(a: number): number;
export function __wbindgen_realloc(a: number, b: number, c: number): number;
export function __wbindgen_free(a: number, b: number): void;
