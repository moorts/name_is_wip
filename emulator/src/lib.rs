mod core;
mod terminator;
mod kreator;
mod utils;

use wasm_bindgen::prelude::*;

use crate::core::emulator::Emulator;
use crate::kreator::assembler::Assembler;
use crate::terminator::disassembler::Disassembler;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// JavaScript functions
#[wasm_bindgen]
extern "C" {
    // JS: alert(msg)
    fn alert(msg: &str);
    
    // JS: console.log(msg)
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

#[wasm_bindgen]
pub fn assemble(code: &str) -> Vec<u8> {
    let asm = Assembler::new(code);
    let result = asm.assemble();
    
    match result {
        Ok(bytes) => {
            return bytes;
        }
        Err(msg) => {
            log("Error while assembling: ");
            log(msg);
        }
    }
    
    return vec![];
}

#[wasm_bindgen]
pub fn disassemble(bytes: Vec<u8>) -> String {
    let mut disassembler = Disassembler::load_bytes(bytes);
    let result = disassembler.disassemble();
    
    match result {
        Ok(code) => {
            return code.join("\n");
        }
        Err(msg) => {
            log("Error while disassembling: ");
            log(msg);
        }
    }
    
    return "".to_string();
}

#[wasm_bindgen]
pub fn createEmulator(memory: Vec<u8>) -> Emulator {
    let mut emu = Emulator::new();
    emu.load_ram(memory, 0);
    return emu;
}