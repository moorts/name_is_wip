mod core;
mod terminator;
mod kreator;
mod utils;

use wasm_bindgen::prelude::*;
use crate::kreator::assembler::Assembler;

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
            log(msg);
        }
    }
    
    return vec![];
}