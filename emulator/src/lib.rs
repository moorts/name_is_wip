mod core;
mod terminator;
mod kreator;
mod utils;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

use crate::core::emulator::Emulator;
use crate::core::io::{InputDevice, OutputDevice};
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

    console_error_panic_hook::set_once();
    
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

#[wasm_bindgen]
pub fn registerSpaceInvadersDevices(emulator: &mut Emulator) {

    let device = Rc::new(RefCell::new(SpaceInvaders::new()));
    
    emulator.register_input_device(device.clone(), 0);
    emulator.register_input_device(device.clone(), 1);
    emulator.register_input_device(device.clone(), 2);
    emulator.register_input_device(device.clone(), 3);

    emulator.register_output_device(device.clone(), 2);
    emulator.register_output_device(device.clone(), 3);
    emulator.register_output_device(device.clone(), 4);
    emulator.register_output_device(device.clone(), 5);
    emulator.register_output_device(device.clone(), 6);
    
}

struct SpaceInvaders {
    shift_register: u16,
    register_offset: u8
}

impl SpaceInvaders {
    fn new() -> Self {
        SpaceInvaders {
            shift_register: 0,
            register_offset: 0
        }
    }
}

impl InputDevice for SpaceInvaders {
    fn read(&self, port: u8) -> u8 {
        match port {
            0 => 0b01110000,
            1 => 0b10010000,
            2 => 0b00000000,
            3 => ((self.shift_register >> self.register_offset) & 0xFF) as u8,
            _ => 0x00
        }
    }
}

impl OutputDevice for SpaceInvaders {
    fn write(&mut self, port: u8, byte: u8) {
        log(format!("SpaceInvaders: write to port {}: {}", port, byte).as_str());
        match port {
            2 => {
                self.register_offset = byte;
            },
            3 => {

            },
            4 => {
                self.shift_register = (self.shift_register >> 8) | (byte as u16) << 8;
            },
            5 => {

            },
            6 => {

            },
            _ => {

            }
        }
    }
}